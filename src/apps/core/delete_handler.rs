use super::App;
use crate::utils::delete_connection::delete_connection;
use crate::utils::scan::scan_networks;

use crossterm::event::KeyEventKind::Press;
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
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.delete_connection();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('Y'),
                    kind: Press,
                    ..
                }) => {
                    self.delete_connection();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('y'),
                    kind: Press,
                    ..
                }) => {
                    self.delete_connection();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('N'),
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_delete_confirmation = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('n'),
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_delete_confirmation = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    kind: Press,
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_delete_confirmation = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_delete_confirmation = false;
                }

                _ => {}
            }
        };
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
        if self.flags.show_saved {
            delete_connection(
                self.saved_connection.connections[self.selected]
                    .ssid
                    .clone(),
            );
        } else {
            delete_connection(self.wifi_list.lock().unwrap()[self.selected].ssid.clone());
        }
        self.flags.show_delete_confirmation = false;
        scan_networks(self.wifi_list.clone());
    }
}