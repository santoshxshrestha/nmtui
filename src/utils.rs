use crate::app::App;
use std::process::Command;

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::try_restore().unwrap();
    app_result
}
pub fn connect_to_network(ssid: &str, password: &str) -> Result<(), Box<dyn std::error::Error>> {
    let status = if password.is_empty() {
        Command::new("nmcli")
            .args(&["dev", "wifi", "connect", ssid])
            .status()?
    } else {
        Command::new("nmcli")
            .args(&["dev", "wifi", "connect", ssid, "password", password])
            .status()?
    };

    if status.success() {
        Ok(())
    } else {
        Err("Failed to connect to the network".into())
    }
}
