use crate::{WifiNetwork, apps::handlers::status::Status};
use std::{
    process::{Command, ExitStatus},
    sync::{Arc, RwLock},
};

// handle the output of the command execution and return a Status object
fn handle_comand_output(output: std::process::Output) -> Status {
    let status = output.status;
    if status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        Status::new(stdout.into(), status)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Status::new(stderr.into(), status)
    }
}

// handle the result of the command execution and return a Status object
fn handle_command_result(result: Result<std::process::Output, std::io::Error>) -> Status {
    match result {
        Ok(output) => handle_comand_output(output),
        Err(e) => Status::new(
            format!("Failed to execute nmcli: {}", e),
            ExitStatus::default(),
        ),
    }
}

pub fn disconnect_connected_network(wifi_list: Arc<RwLock<Vec<WifiNetwork>>>) -> Status {
    let list = wifi_list.read().expect("WifiNetworks lock poisoned");

    for network in list.iter() {
        if network.in_use {
            let ssid = &network.ssid;
            let output = Command::new("nmcli")
                .args(["connection", "down", ssid])
                .output();
            // we are not returning any thign if hte network disconnected successfully,
            // but if there is an error we will return the error message
            handle_command_result(output);
        }
    }
    Status::new(
        "No connected network found".to_string(),
        ExitStatus::default(),
    )
}
