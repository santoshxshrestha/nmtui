use std::process::Command;

pub fn saved_connections() -> Vec<String> {
    // nmcli -t -f NAME,TYPE connection show
    let output = Command::new("nmcli")
        .args(["-t", "-f", "NAME,TYPE", "connection", "show"])
        .output()
        .expect(" Failed to execute nmcli command");

    let mut ssids: Vec<String> = Vec::new();
    if !output.status.success() {
        return ssids;
    }
    let stdout = String::from_utf8_lossy(&output.stdout);

    // header is already handled by nmcli with -t flag
    for line in stdout.lines() {
        let mut parts = line.splitn(2, ':');
        let ssid = parts.next().unwrap_or("").to_string();
        let connection_type = parts.next().unwrap_or("").to_string();

        // we are only interested in wifi saved connections
        if !ssid.is_empty() && !connection_type.is_empty() && connection_type == "802-11-wireless" {
            ssids.push(ssid);
        }
    }
    ssids
}
