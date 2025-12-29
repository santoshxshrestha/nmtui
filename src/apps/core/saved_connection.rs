use super::App;

use crossterm::event::poll;
use crossterm::event::{self, Event, KeyEvent};
use std::process::Command;
use std::{io, time::Duration};

#[derive(Debug)]
pub struct Connections {
    pub ssid: String,
    pub last_used: String,
}

#[derive(Debug, Default)]
pub struct SavedConnections {
    pub connections: Vec<Connections>,
    pub selected_index: usize,
}

impl SavedConnections {
    // nmcli -t -f NAME,TYPE,TIMESTAMP-REAL connection show
    pub fn fetch_saved_connections(&mut self) {
        let output = Command::new("nmcli")
            .args(["-t", "-f", "NAME,TYPE,TIMESTAMP-REAL", "connection", "show"])
            .output()
            .expect(" Failed to execute nmcli command");

        // this will store the connections
        let mut connections: Vec<Connections> = Vec::new();
        if !output.status.success() {
            self.connections = connections;
            return;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);

        for line in stdout.lines() {
            // there is some thing like \: in the time stamp, so we limit split to 3 parts
            let mut parts = line.splitn(3, ':');
            let ssid = parts.next().unwrap_or("").to_string();

            // if the connection type is empty, it means its unsecured
            let connection_type = parts
                .next()
                .map(|s| {
                    if s.is_empty() {
                        "Unsecured".to_string()
                    } else {
                        s.to_string()
                    }
                })
                .unwrap_or("".to_string());

            let last_used = parts.next().unwrap_or("").to_string();

            if !ssid.is_empty()
                && !connection_type.is_empty()
                && connection_type == "802-11-wireless"
            {
                connections.push(Connections { ssid, last_used });
            }
        }
        self.connections = connections;
    }
}

impl App {
    pub fn handle_saved(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: event::KeyCode::Esc,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_saved_list();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('d'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {}

                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('j'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.update_selected_saved_network(1);
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Down,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.update_selected_saved_network(1);
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('k'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.update_selected_saved_network(-1);
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Up,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.update_selected_saved_network(-1);
                }
                _ => {}
            };
        }
        Ok(())
    }
    pub fn update_selected_saved_network(&mut self, direction: isize) {
        let len = self.saved_connection.connections.len();
        // if there is some content in the saved list
        if len > 0 {
            self.saved_connection.selected_index =
                ((self.saved_connection.selected_index as isize + direction)
                    .rem_euclid(len as isize)) as usize;
        }
    }
    pub fn open_saved_list(&mut self) {
        self.saved_connection.fetch_saved_connections();
        self.show_saved = true;
    }

    pub fn close_saved_list(&mut self) {
        self.show_saved = false;
    }
}
