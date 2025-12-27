use crate::apps::handlers::WifiInputState;
use crate::apps::handlers::flags::Flags;
use crate::apps::handlers::status::Status;
use std::process::{Command, ExitStatus};

pub fn connect_to_network(wifi_creadentials: &WifiInputState) -> Status {
    let WifiInputState {
        flags: Flags { is_hidden, .. },
        ssid,
        password,
        ..
    } = wifi_creadentials;

    let output = if password.is_empty() {
        // connecting to the open network or the network that was already saved
        Command::new("nmcli")
            .args(["dev", "wifi", "connect", ssid])
            .output()
    } else if *is_hidden {
        // connecting to the hidden network
        Command::new("nmcli")
            .args([
                "dev", "wifi", "connect", ssid, "password", password, "hidden", "yes",
            ])
            .output()
    } else {
        // connecting to the secured network with password
        Command::new("nmcli")
            .args(["dev", "wifi", "connect", ssid, "password", password])
            .output()
    };

    match output {
        Ok(output) => {
            let status = output.status;
            if status.success() {
                // here this thing is creating some glitch in the ui when connecting successfully
                // let stdout = String::from_utf8_lossy(&output.stdout);
                let stdout = format!("Successfully connected to '{}'", ssid);
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
