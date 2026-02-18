use super::App;
use crate::utils::delete_connection::delete_connection;
use crate::utils::scan::scan_networks;

use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::io;
use std::time::Duration;

impl App {
    /// Handle a pending delete-confirmation keyboard event when the confirmation overlay is shown.
    ///
    /// Processes at most one terminal event (polled non-blocking). On Enter, 'Y', or 'y' it deletes
    /// the currently selected connection; on 'N', 'n', Esc, or 'q' it hides the delete confirmation;
    /// on Ctrl-C it exits the application. Other events are ignored.
    ///
    /// # Returns
    ///
    /// `Ok(())` if the event was processed (or if no event was available), or an `io::Error` if a
    /// terminal poll/read operation fails.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create or obtain a mutable App instance, then handle any pending delete confirmation input.
    /// // let mut app = App::new(...);
    /// // let _ = app.handle_delete_confirmation();
    /// ```
    pub fn handle_delete_confirmation(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))?
            && let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
        {
            match (code, modifiers) {
                (KeyCode::Enter, _) | (KeyCode::Char('Y'), _) | (KeyCode::Char('y'), _) => {
                    self.delete_connection();
                }
                (KeyCode::Char('N'), _)
                | (KeyCode::Char('n'), _)
                | (KeyCode::Esc, _)
                | (KeyCode::Char('q'), _) => {
                    self.flags.show_delete_confirmation = false;
                }
                (KeyCode::Char('c'), event::KeyModifiers::CONTROL) => {
                    self.exit();
                }
                _ => {}
            }
        }
        Ok(())
    }
    /// Delete the currently selected connection and refresh the network list.
    ///
    /// If the app is showing saved connections, deletes the selected saved connection; otherwise deletes
    /// the selected entry from the scanned Wi‑Fi list. After deletion, clears the delete-confirmation
    /// flag and triggers a network rescan to refresh `wifi_list`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// let mut app = App::default();
    /// app.selected = 0;
    /// app.flags.show_saved = true;
    /// app.delete_connection();
    /// ```
    pub fn delete_connection(&mut self) {
        // Determine which connection to delete based on the current view
        //
        // here this one will delete from the saved connections list
        if self.flags.show_saved {
            delete_connection(
                self.saved_connection.connections[self.saved_connection.selected_index]
                    .ssid
                    .clone(),
            );
            self.reset_saved_selection();
            self.saved_connection.fetch_saved_connections();
        } else {
            // this one will delete the connection from the wifi list
            delete_connection(
                self.wifi_list.read().expect("WifiNetworks lock poisoned")[self.selected]
                    .ssid
                    .clone(),
            );
            self.reset_selection();
            scan_networks(self.wifi_list.clone(), self.flags.is_scanning.clone());
        }
        self.flags.show_delete_confirmation = false;
    }
}
