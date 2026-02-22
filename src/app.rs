pub const MAX_PASSWORD_LEN: usize = 256;
pub const MASK_CHAR: char = '●';

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Locked,
    Authenticating,
    Failed(u32),
    Unlocked,
}

pub struct App {
    pub password: String,
    
}
