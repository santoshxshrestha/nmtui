use crate::WifiNetwork;
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn scan_networks(wifi_list: Arc<Mutex<Vec<WifiNetwork>>>) {
    // // nmcli -t -f IN-USE,SSID,SECURITY device wifi list
    thread::spawn(move || {
        let mut wifi_list_lock = wifi_list.lock().unwrap();
        let output = Command::new("nmcli")
            .args(["-t", "-f", "IN-USE,SSID,SECURITY", "device", "wifi", "list"])
            .output()
            .expect("Failed to execute nmcli command");

        if !output.status.success() {
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut networks: Vec<WifiNetwork> = Vec::new();

        for line in stdout.lines() {
            let mut parts = line.splitn(3, ':');

            let in_use = parts.next() == Some("*");
            let ssid = parts.next().unwrap_or("").to_string();
            let security = parts.next().unwrap_or("--").to_string();
            if !ssid.is_empty() {
                networks.push(WifiNetwork {
                    in_use,
                    ssid,
                    security,
                })
            }
        }
        networks.push(WifiNetwork {
            in_use: false,
            ssid: "Connect to Hidden network".to_string(),
            security: "?".to_string(),
        });
        // networks.retain(|network| !network.ssid.is_empty());
        *wifi_list_lock = networks;
    });
}
