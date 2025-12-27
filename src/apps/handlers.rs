mod flags;
mod password_handler;
mod ssid_handler;
mod status;
mod utils;
use crate::utils::connect::connect_to_network;
use flags::Flags;
use status::Status;

#[derive(Debug, Default)]
pub struct WifiInputState {
    pub ssid: String,
    pub password: String,
    pub cursor_pos: u16,
    pub status: Status,
    pub flags: Flags,
}

impl WifiInputState {
    fn prepare_to_connect(&mut self) {
        self.flags.show_password_popup = false;
        self.status = connect_to_network(self);
        self.reset_cursor_position();
        self.flags.is_hidden = false;
        self.flags.show_status_popup = true;
    }

    fn move_cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    fn reset_cursor_position(&mut self) {
        self.cursor_pos = 0;
    }
}
