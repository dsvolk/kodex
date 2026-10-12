//! Forward explicit programs while leaving entitlement and model policy to the server.

use kodex_api::AccessPrograms;
use kodex_features::Feature;
use kodex_login::KodexAuth;
use kodex_model_provider_info::OPENAI_PROVIDER_ID;
use kodex_protocol::error::KodexErr;
use kodex_protocol::error::Result;
use kodex_protocol::turn_input::CyberAccessProgram;

#[derive(Clone, Copy, Debug)]
pub(crate) enum ApiKeyCyberAccessPrograms {
    UnsupportedProvider,
    Disabled,
    Enabled,
}

impl ApiKeyCyberAccessPrograms {
    pub(crate) fn from_config(config: &crate::config::Config) -> Self {
        if config.model_provider_id != OPENAI_PROVIDER_ID {
            Self::UnsupportedProvider
        } else if config.features.enabled(Feature::ApiKeyCyberAccessPrograms) {
            Self::Enabled
        } else {
            Self::Disabled
        }
    }
}

pub(crate) fn for_provider(
    provider_id: &str,
    program: Option<CyberAccessProgram>,
) -> Option<CyberAccessProgram> {
    program.filter(|_| provider_id == OPENAI_PROVIDER_ID)
}

pub(crate) fn for_auth(
    auth: Option<&KodexAuth>,
    program: Option<CyberAccessProgram>,
    policy: ApiKeyCyberAccessPrograms,
) -> Result<Option<AccessPrograms>> {
    let Some(program) = program else {
        return Ok(None);
    };
    let Some(auth) = auth else {
        return Ok(None);
    };
    if auth.is_chatgpt_auth() {
        return Ok(Some(program.into()));
    }
    if !auth.is_api_key_auth() {
        return Ok(None);
    }
    match policy {
        ApiKeyCyberAccessPrograms::UnsupportedProvider => Ok(None),
        ApiKeyCyberAccessPrograms::Disabled => Err(KodexErr::InvalidRequest(
            "Cyber access programs are disabled for this API-key session.".to_owned(),
        )),
        ApiKeyCyberAccessPrograms::Enabled => Ok(Some(program.into())),
    }
}
