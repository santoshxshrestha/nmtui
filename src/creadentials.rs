#[derive(Debug)]
pub struct WifiCredentials {
    pub is_hidden: bool,
    pub ssid: String,
    pub password: String,
    pub cursor_pos: u16,
}

impl Default for WifiCredentials {
    fn default() -> Self {
        WifiCredentials {
            is_hidden: false,
            ssid: String::new(),
            password: String::new(),
            cursor_pos: 0,
        }
    }
}
