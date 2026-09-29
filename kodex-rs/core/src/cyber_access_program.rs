use kodex_api::AccessPrograms;
use kodex_login::KodexAuth;
use kodex_protocol::turn_input::CyberAccessProgram;

pub(crate) fn for_auth(
    auth: Option<&KodexAuth>,
    program: Option<CyberAccessProgram>,
) -> Option<AccessPrograms> {
    program
        .filter(|_| auth.is_some_and(KodexAuth::is_chatgpt_auth))
        .map(AccessPrograms::from)
}
