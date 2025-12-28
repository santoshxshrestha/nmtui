use std::process::Command;
use std::thread;

pub fn delete_connection(ssid: String) {
    // nmcli connection delete id "<SSID>"
    thread::spawn(move || {
        let output = Command::new("nmcli")
            .args(["connection", "delete", "id", ssid.trim()])
            .output()
            .expect(" Failed to execute nmcli command");
        if !output.status.success() {}
    });
}
