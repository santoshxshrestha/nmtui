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

            // this is the connection type not the security type
            let connection_type = parts.next().unwrap_or("").to_string();

            let last_used = parts
                .next()
                .unwrap_or("")
                .chars()
                .filter(|&c| c != '\\')
                .collect();

            if !ssid.is_empty() && connection_type == "802-11-wireless" {
                connections.push(Connections { ssid, last_used });
            }
        }
        self.connections = connections;
    }
}

impl App {
    /// Handle keyboard input when the saved-connections UI is active and update the application state.
    ///
    /// Recognizes key presses and performs the following actions:
    /// - 'q' or Esc: close the saved-connections view
    /// - Ctrl+C: exit the application
    /// - 'd': show the delete-confirmation dialog
    /// - 'j' or Down: advance the saved-connection selection by one
    /// - 'k' or Up: move the saved-connection selection back by one
    /// - 'h' or '?': show the help view
    /// - Ctrl+R: refresh the saved connections list by re-fetching saved connections
    ///
    /// # Returns
    ///
    /// `Ok(())` on success, or an `io::Error` if polling or reading terminal input fails.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::io;
    /// # // `App` must be constructed according to the surrounding codebase.
    /// # let mut app = App::default();
    /// // Process any pending saved-list input once.
    /// let _ = app.handle_saved();
    /// ```
    pub fn handle_saved(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('q'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_saved_list();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Esc,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_saved_list();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('c'),
                    kind: event::KeyEventKind::Press,
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('d'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    // this will evaluate to run the delete confirmation dialog from the core ui
                    self.flags.show_delete_confirmation = true;
                }

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
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('h'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.flags.show_help = true;
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('?'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.flags.show_help = true;
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('r'),
                    kind: event::KeyEventKind::Press,
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.saved_connection.fetch_saved_connections();
                }
                _ => {}
            };
        }
        Ok(())
    }
    /// Move the selected saved network index by `direction`, wrapping around the list bounds.
    ///
    /// If the saved-connections list is empty, the selection is unchanged.
    ///
    /// # Parameters
    ///
    /// - `direction`: Signed offset to apply to the current `selected_index`. A positive value moves the selection forward, a negative value moves it backward. The resulting index wraps within `[0, len - 1]`.
    ///
    /// # Examples
    ///
    /// ```
    /// // wrap-forward example: index 2 in a list of length 3 + direction 1 -> 0
    /// let idx = 2usize;
    /// let len = 3usize;
    /// let new = ((idx as isize + 1).rem_euclid(len as isize)) as usize;
    /// assert_eq!(new, 0);
    /// ```
    pub fn update_selected_saved_network(&mut self, direction: isize) {
        let len = self.saved_connection.connections.len();
        // if there is some content in the saved list
        if len > 0 {
            self.saved_connection.selected_index =
                ((self.saved_connection.selected_index as isize + direction)
                    .rem_euclid(len as isize)) as usize;
        }
    }
    /// Opens the saved-connections view.
    ///
    /// This refreshes the stored list of saved Wi‑Fi connections and makes the saved-connections UI visible.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Construct your application, refresh saved connections, and show the saved list:
    /// let mut app = App::new();
    /// app.open_saved_list();
    /// assert!(app.flags.show_saved);
    /// ```
    pub fn open_saved_list(&mut self) {
        self.saved_connection.fetch_saved_connections();
        self.flags.show_saved = true;
    }

    /// Hides the saved-connections view in the application's UI.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = App::default();
    /// app.flags.show_saved = true;
    /// app.close_saved_list();
    /// assert!(!app.flags.show_saved);
    /// ```
    pub fn close_saved_list(&mut self) {
        self.flags.show_saved = false;
    }

    pub fn reset_saved_selection(&mut self) {
        self.saved_connection.selected_index = 0;
    }
}
