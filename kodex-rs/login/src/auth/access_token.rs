const PERSONAL_ACCESS_TOKEN_PREFIX: &str = "at-";

pub(super) enum KodexAccessToken<'a> {
    PersonalAccessToken(&'a str),
    AgentIdentityJwt(&'a str),
}

pub(super) fn classify_kodex_access_token(access_token: &str) -> KodexAccessToken<'_> {
    if access_token.starts_with(PERSONAL_ACCESS_TOKEN_PREFIX) {
        KodexAccessToken::PersonalAccessToken(access_token)
    } else {
        KodexAccessToken::AgentIdentityJwt(access_token)
    }
}

#[cfg(test)]
#[path = "access_token_tests.rs"]
mod tests;
