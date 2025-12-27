use std::process::Command;
use std::thread;

pub struct SavedConnection {
    pub ssid: String,
    pub connection_type: String,
}

pub fn saved_connections() {
    thread::spawn(move || {
        let output = Command::new("nmcli")
            .args(["-f", "NAME,TYPE", "connection", "show"])
            .output()
            .expect(" Failed to execute nmcli command");
        if !output.status.success() {
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut networks: Vec<SavedConnection> = Vec::new();
        for line in stdout.lines() {
            let mut parts = line.splitn(2, ' ');
            let ssid = parts.next().unwrap_or("").to_string();
            let connection_type = parts.next().unwrap_or("").to_string();
            if !ssid.is_empty() && !connection_type.is_empty() && connection_type == "wifi" {
                networks.push(SavedConnection {
                    ssid,
                    connection_type,
                });
            }
        }
    });
}
