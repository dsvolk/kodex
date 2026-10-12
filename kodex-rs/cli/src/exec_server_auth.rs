//! CLI-only glue between Kodex authentication and generic AWS signing.
//!
//! Keeping this adapter here leaves `kodex-api` and `kodex-aws-auth` independent.

use std::sync::Arc;

use http::HeaderMap;
use kodex_api::AuthError;
use kodex_api::AuthProvider;
use kodex_api::SharedAuthProvider;
use kodex_aws_auth::AwsAuthConfig;
use kodex_aws_auth::AwsAuthContext;
use kodex_aws_auth::AwsAuthError;
use kodex_aws_auth::AwsRequestToSign;
use kodex_http_client::Request;
use kodex_http_client::RequestBody;
use kodex_http_client::RequestCompression;

/// Creates a SigV4 provider, preferring an explicit profile over the default credential chain.
pub(super) async fn aws_sigv4_auth_provider(
    mut config: AwsAuthConfig,
    http_client_factory: kodex_http_client::HttpClientFactory,
) -> Result<SharedAuthProvider, AwsAuthError> {
    config.profile = config
        .profile
        .map(|profile| profile.trim().to_string())
        .filter(|profile| !profile.is_empty());
    config.region = config
        .region
        .map(|region| region.trim().to_string())
        .filter(|region| !region.is_empty());
    let context = if config.profile.is_some() {
        AwsAuthContext::load_profile(config, http_client_factory).await
    } else {
        AwsAuthContext::load(config, http_client_factory).await
    }?;
    Ok(Arc::new(AwsSigV4AuthProvider { context }))
}

#[derive(Debug)]
struct AwsSigV4AuthProvider {
    context: AwsAuthContext,
}

impl AuthProvider for AwsSigV4AuthProvider {
    fn add_auth_headers(&self, _headers: &mut HeaderMap) {}

    fn apply_auth(&self, mut request: Request) -> kodex_api::AuthProviderFuture<'_> {
        Box::pin(async move {
            let prepared = request.prepare_body_for_send().map_err(AuthError::Build)?;
            let signed = self
                .context
                .sign(AwsRequestToSign {
                    method: request.method.clone(),
                    url: request.url.clone(),
                    headers: prepared.headers.clone(),
                    body: prepared.body_bytes(),
                })
                .await
                .map_err(|error| {
                    if error.is_retryable() {
                        AuthError::Transient(error.to_string())
                    } else {
                        AuthError::Build(error.to_string())
                    }
                })?;
            request.url = signed.url;
            request.headers = signed.headers;
            request.body = prepared.body.map(RequestBody::Raw);
            request.compression = RequestCompression::None;
            Ok(request)
        })
    }
}

#[cfg(test)]
#[path = "exec_server_auth_tests.rs"]
mod tests;
