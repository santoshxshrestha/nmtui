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
) -> Result<String, Box<dyn std::error::Error>> {
    let WifiCredentials {
        is_hidden,
        ssid,
        password,
        ..
    } = wifi_creadentials;

    let output = if password.is_empty() {
        Command::new("nmcli")
            .args(&["dev", "wifi", "connect", &wifi_creadentials.ssid])
            .output()?
    } else if *is_hidden == true {
        Command::new("nmcli")
            .args(&[
                "dev", "wifi", "connect", ssid, "password", password, "hidden", "yes",
            ])
            .output()?
    } else {
        Command::new("nmcli")
            .args(&["dev", "wifi", "connect", ssid, "password", password])
            .output()?
    };

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Ok(stdout.into())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.into())
    }
}
