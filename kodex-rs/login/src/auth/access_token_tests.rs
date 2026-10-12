use super::*;

#[test]
fn classifies_personal_access_tokens_by_prefix() {
    assert!(matches!(
        classify_kodex_access_token("at-example"),
        KodexAccessToken::PersonalAccessToken("at-example")
    ));
    assert!(matches!(
        classify_kodex_access_token("header.payload.signature"),
        KodexAccessToken::AgentIdentityJwt("header.payload.signature")
    ));
}
