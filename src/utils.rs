use crate::WifiCredentials;
use crate::app::App;
use std::process::Command;

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::try_restore().unwrap();
    app_result
}

pub fn connect_to_network(
    wifi_creadentials: &WifiCredentials,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = if wifi_creadentials.password.is_empty() {
        Command::new("nmcli")
            .args(&["dev", "wifi", "connect", &wifi_creadentials.ssid])
            .status()?
    } else if wifi_creadentials.is_hidden == true {
        Command::new("nmcli")
            .args(&[
                "dev",
                "wifi",
                "connect",
                &wifi_creadentials.ssid,
                "password",
                &wifi_creadentials.password,
                "hidden",
                "yes",
            ])
            .status()?
    } else {
        Command::new("nmcli")
            .args(&[
                "dev",
                "wifi",
                "connect",
                &wifi_creadentials.ssid,
                "password",
                &wifi_creadentials.password,
            ])
            .status()?
    };

    if status.success() {
        Ok(())
    } else {
        Err("Failed to connect to the network".into())
    }
}
