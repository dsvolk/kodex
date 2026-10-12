//! Composes primary provider credentials with independently managed gateway OAuth credentials.

use std::sync::Arc;

use crate::auth::ResolvedProviderAuth;
use http::HeaderMap;
use http::HeaderName;
use http::HeaderValue;
use kodex_api::AuthError;
use kodex_api::AuthHeadersFuture;
use kodex_api::AuthProvider;
use kodex_api::AuthProviderFuture;
use kodex_api::SharedAuthProvider;
use kodex_login::GatewayAuthError;
use kodex_login::GatewayAuthManager;
use kodex_model_provider_info::GatewayOAuthConfig;
use kodex_model_provider_info::GatewayOAuthDelivery;
use kodex_model_provider_info::ModelProviderInfo;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::Result;

pub(crate) async fn compose_auth(
    provider: &ModelProviderInfo,
    manager: Option<&std::result::Result<Arc<GatewayAuthManager>, String>>,
    mut resolved: ResolvedProviderAuth,
) -> Result<ResolvedProviderAuth> {
    let Some(config) = provider.gateway_oauth.as_ref() else {
        return Ok(resolved);
    };
    provider.validate().map_err(KodexErr::InvalidRequest)?;
    let manager = manager.ok_or_else(|| {
        KodexErr::InvalidRequest("gateway_oauth requires auth runtime configuration".into())
    })?;
    let manager = manager
        .as_ref()
        .map_err(|error| KodexErr::InvalidRequest(error.clone()))?;
    let token = manager.resolve_access_token().await.map_err(|error| {
        if let Some(login_error) = error
            .get_ref()
            .and_then(|source| source.downcast_ref::<GatewayAuthError>())
        {
            return KodexErr::InvalidRequest(login_error.to_string());
        }
        // Issuer errors may echo arbitrary credentials from configured URLs. Keep the
        // diagnostic safe and bounded, preserving retries for transient failures.
        KodexErr::Io(std::io::Error::new(
            error.kind(),
            "Gateway OAuth authentication failed; check the gateway configuration and credential store.",
        ))
    })?;
    let (name, value) = gateway_header(config, &token)?;
    if resolved.auth.to_auth_headers().contains_key(&name) {
        return Err(KodexErr::InvalidRequest(
            "gateway OAuth conflicts with primary auth headers".into(),
        ));
    }
    resolved.auth = Arc::new(CombinedAuth {
        primary: resolved.auth,
        name,
        value,
    });
    Ok(resolved)
}

fn gateway_header(config: &GatewayOAuthConfig, token: &str) -> Result<(HeaderName, HeaderValue)> {
    if token.is_empty() {
        return Err(KodexErr::InvalidRequest(
            "gateway OAuth returned an empty token".into(),
        ));
    }
    let (name, value) = match &config.delivery {
        GatewayOAuthDelivery::Header { name, scheme } => (
            HeaderName::from_bytes(name.as_bytes())
                .map_err(|_| KodexErr::InvalidRequest("invalid gateway header".into()))?,
            format!("{scheme} {token}"),
        ),
        GatewayOAuthDelivery::Cookie { name } => {
            // RFC 6265 cookie-octet excludes separators, quotes, whitespace and non-ASCII.
            if !token.bytes().all(
                |byte| matches!(byte, 0x21 | 0x23..=0x2b | 0x2d..=0x3a | 0x3c..=0x5b | 0x5d..=0x7e),
            ) {
                return Err(KodexErr::InvalidRequest(
                    "gateway OAuth token is not a valid cookie value".into(),
                ));
            }
            (http::header::COOKIE, format!("{name}={token}"))
        }
    };
    let mut value = HeaderValue::from_str(&value)
        .map_err(|_| KodexErr::InvalidRequest("invalid gateway OAuth token header".into()))?;
    value.set_sensitive(true);
    Ok((name, value))
}

struct CombinedAuth {
    primary: SharedAuthProvider,
    name: HeaderName,
    value: HeaderValue,
}

impl AuthProvider for CombinedAuth {
    fn add_auth_headers(&self, headers: &mut HeaderMap) {
        self.primary.add_auth_headers(headers);
        headers
            .entry(self.name.clone())
            .or_insert(self.value.clone());
    }

    fn resolve_auth_headers(&self) -> AuthHeadersFuture<'_> {
        Box::pin(async move {
            let mut headers = self.primary.resolve_auth_headers().await?;
            if headers.contains_key(&self.name) {
                return Err(AuthError::Build(
                    "gateway OAuth conflicts with primary auth headers".into(),
                ));
            }
            headers.insert(self.name.clone(), self.value.clone());
            Ok(headers)
        })
    }

    fn apply_auth(&self, request: kodex_http_client::Request) -> AuthProviderFuture<'_> {
        Box::pin(async move {
            let mut request = self.primary.apply_auth(request).await?;
            if request.headers.contains_key(&self.name) {
                return Err(AuthError::Build(
                    "gateway OAuth conflicts with request headers".into(),
                ));
            }
            request
                .headers
                .insert(self.name.clone(), self.value.clone());
            Ok(request)
        })
    }
}
