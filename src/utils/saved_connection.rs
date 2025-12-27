use std::process::Command;
use std::thread;

pub struct SavedConnection {
    pub ssid: String,
    pub connection_type: String,
}

pub fn saved_connections() {
    // nmcli -t -f NAME,TYPE connection show
    thread::spawn(move || {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
            .output()
            .expect(" Failed to execute nmcli command");
        if !output.status.success() {
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut networks: Vec<SavedConnection> = Vec::new();

        // header is already handled by nmcli with -t flag
        for line in stdout.lines() {
            let mut parts = line.splitn(2, ':');
            let ssid = parts.next().unwrap_or("").to_string();
            let connection_type = parts.next().unwrap_or("").to_string();

            // we are only interested in wifi saved connections
            if !ssid.is_empty()
                && !connection_type.is_empty()
                && connection_type == "802-11-wireless"
            {
                networks.push(SavedConnection {
                    ssid,
                    connection_type,
                });
            }
        }
    });
}
