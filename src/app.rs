pub const MAX_PASSWORD_LEN: usize = 256;
pub const SHAKE_FRAMES: usize = 6;
pub const SHAKE_OFFSETS: [i32; SHAKE_FRAMES] = [2, -1, 3, -3, 2, -1];

pub const MASK_CHAR: char = '●';

pub const TICK_RATE_MS: u64 = 120;
pub const APP_NAME: &str = "login";

#[derive(Debug, Clone, PartialEq)]
pub enum AppState {
    Locked,
    Authenticating,
    Failed(u32),
    Unlocked,
}

pub struct App {
    pub password: String,
    pub state: AppState,
    pub shake_frame: usize,
    pub tick_count: u64,
    pub fail_count: u32,
}

impl App {
    pub fn new() -> Self {
        App {
            password: String::with_capacity(64),
            state: AppState::Locked,
            shake_frame: 0,
            tick_count: 0,
            fail_count: 0,
        }
    }

    pub fn push_char(&mut self, c: char) {
        if self.password.len() < MAX_PASSWORD_LEN {
            self.password.push(c);
        }
    }

    pub fn pop_char(&mut self) {
        self.password.pop();
    }

    pub fn begin_auth(&mut self) -> String {
        let pwd = self.password.clone();
        self.password.clear();
        self.state = AppState::Authenticating;
        pwd
    }

    pub fn on_auth_success(&mut self) {
        self.state = AppState::Unlocked;
    }

    pub fn on_auth_failure(&mut self) {
        self.fail_count += 1;
        self.state = AppState::Failed(self.fail_count);
        self.shake_frame = 0;
    }

    pub fn tick(&mut self) {
        self.tick_count = self.tick_count.wrapping_add(1);

        if matches!(self.state, AppState::Failed(_)) {
            self.shake_frame += 1;
        }
    }

    pub fn should_quit(&self) -> bool {
        self.state == AppState::Unlocked
    }

    pub fn shake_offset(&self) -> i32 {
        if matches!(self.state, AppState::Failed(_)) && self.shake_frame < SHAKE_FRAMES {
            SHAKE_OFFSETS[self.shake_frame]
        } else {
            0
        }
    }
}
