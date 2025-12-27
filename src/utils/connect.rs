use std::process::{Command, ExitStatus};


pub fn connect_to_network(wifi_creadentials: &WifiCredentials) -> Status {
    let WifiCredentials {
        flags: Flags { is_hidden, .. },
        ssid,
        password,
        ..
    } = wifi_creadentials;

    let output = if password.is_empty() {
        Command::new("nmcli")
            .args(["dev", "wifi", "connect", ssid])
            .output()
    } else if *is_hidden {
        Command::new("nmcli")
            .args([
                "dev", "wifi", "connect", ssid, "password", password, "hidden", "yes",
            ])
            .output()
    } else {
        Command::new("nmcli")
            .args(["dev", "wifi", "connect", ssid, "password", password])
            .output()
    };

    match output {
        Ok(output) => {
            let status = output.status;
            if status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                Status::new(stdout.to_string(), status)
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                Status::new(stderr.to_string(), status)
            }
        }
        Err(e) => Status::new(
            format!("Failed to execute nmcli: {}", e),
            ExitStatus::default(),
        ),
    }
}
