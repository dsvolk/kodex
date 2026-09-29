//! Derives the enterprise credential owner from the active Kodex account.

use kodex_config::McpEmaAuthScope;
use kodex_login::KodexAuth;

pub fn ema_auth_scope(auth: Option<&KodexAuth>) -> Option<McpEmaAuthScope> {
    let auth = auth?;
    McpEmaAuthScope::new(auth.get_chatgpt_user_id()?, auth.get_account_id()?)
}
