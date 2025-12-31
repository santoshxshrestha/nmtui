use crate::{WifiNetwork, apps::handlers::status::Status};
use std::{
    process::{Command, ExitStatus},
    sync::{Arc, Mutex},
};

pub fn disconnect_connected_network(wifi_list: Arc<Mutex<Vec<WifiNetwork>>>) -> Status {
    let list = wifi_list.lock().expect("WifiNetworks lock poisoned");

    for network in list.iter() {
        if network.in_use {
            let ssid = &network.ssid;
            let output = Command::new("nmtui")
                .args(["connection", "down", ssid])
                .output();
            match output {
                Ok(output) => {
                    let status = output.status;
                    if status.success() {
                        let stdout = String::from_utf8_lossy(&output.stdout);
                        return Status::new(stdout.into(), status);
                    } else {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        return Status::new(stderr.into(), status);
                    }
                }
                Err(e) => {
                    return Status::new(
                        format!("Failed to execute nmcli: {}", e),
                        ExitStatus::default(),
                    );
                }
            }
        }
    }
    return Status::new(format!("No connection found"), ExitStatus::default());
}
