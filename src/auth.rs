use pam::Client;
use pam::PasswordConv;

pub const APP_NAME: &str = "login";

#[derive(Debug)]
pub enum AuthResult {
    Success,
    Failure(String),
}

pub fn try_authenticate(username: &str, password: String) -> AuthResult {
    let mut client = match Client::<PasswordConv>::with_password(APP_NAME) {
        Ok(c) => c,
        Err(e) => return AuthResult::Failure(format!("PAM init error: {}", e)),
    };

    client
        .conversation_mut()
        .set_credentials(username, password);

    if let Err(e) = client.authenticate() {
        return AuthResult::Failure(format!("Authentication falied: {}", e));
    }

    if let Err(e) = client.open_session() {
        return AuthResult::Failure(format!("Session open failed: {}", e));
    }
    AuthResult::Success
}
