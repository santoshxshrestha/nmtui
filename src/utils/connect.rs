use crate::apps::handlers::{WifiInputState, flags::Flags, status::Status};
use std::io::Error;
use std::process::{Command, ExitStatus, Output};

// handle the output of the command execution and return a Status object
fn handle_command_output(output: Output, ssid: &str) -> Status {
    let status = output.status;
    if status.success() {
        let stdout = format!("Successfully connected to '{}'", ssid);
        Status::new(stdout.to_string(), status)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Status::new(stderr.to_string(), status)
    }
}

// handle the result of the command execution and return a Status object
fn handle_command_result(result: Result<Output, Error>, ssid: &str) -> Status {
    match result {
        Ok(output) => handle_command_output(output, ssid),
        Err(e) => Status::new(
            format!("Failed to execute nmcli: {}", e),
            ExitStatus::default(),
        ),
    }
}

// Connect to a saved network without password
pub fn connect_to_saved_network(ssid: &str) -> Status {
    let output = Command::new("nmcli")
        .args(["dev", "wifi", "connect", ssid])
        .output();
    handle_command_result(output, ssid)
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

    handle_command_result(output, ssid)
}
