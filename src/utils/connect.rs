use crate::apps::handlers::WifiInputState;
use crate::apps::handlers::flags::Flags;
use crate::apps::handlers::status::Status;
use std::process::{Command, ExitStatus};

// Connect to a saved network without password
pub fn connect_to_saved_network(ssid: &str) -> Status {
    let output = Command::new("nmcli")
        .args(["dev", "wifi", "connect", ssid])
        .output();
    match output {
        Ok(output) => {
            let status = output.status;
            if status.success() {
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

// Connect to a network with given credentials
pub fn connect_to_network(wifi_creadentials: &WifiInputState) -> Status {
    let WifiInputState {
        flags: Flags { is_hidden, .. },
        ssid,
        password,
        ..
    } = wifi_creadentials;

    let output = if *is_hidden {
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
