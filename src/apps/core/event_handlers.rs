use super::App;
use crate::utils::scan::scan_networks;

use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use std::io;
use std::time::Duration;

impl App {
    /// Process a single terminal input event if one is available.
    ///
    /// Reads a pending crossterm event (non-blocking) and dispatches it to update
    /// application state or trigger actions (navigation, connect/disconnect, help,
    /// scan, exit, etc.). If no event is ready the method does nothing.
    ///
    /// # Errors
    ///
    /// Returns any I/O error produced by the underlying event poll/read operations.
    ///
    /// # Examples
    ///
    /// ```
    /// // Create or obtain a mutable App instance and call the handler once.
    /// // The real App type may require different construction; this shows the
    /// // typical usage pattern.
    /// let mut app = App::default();
    /// app.handle_events().unwrap();
    /// ```
    pub fn handle_events(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Char('h'),
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_help = true;
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char('?'),
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_help = true;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    scan_networks(self.wifi_list.clone(), self.flags.is_scanning.clone());
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.prepare_to_connect();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('o'),
                    kind: Press,
                    ..
                }) => {
                    self.prepare_to_connect();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(-1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(1);
                }
                // vim style
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(-1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('d'),
                    kind: Press,
                    ..
                }) => {
                    if self
                        .wifi_list
                        .read()
                        .expect("Wifi list lock poisoned while deleting")[self.selected]
                        .is_saved
                    {
                        self.flags.show_delete_confirmation = true;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('s'),
                    kind: Press,
                    ..
                }) => {
                    self.open_saved_list();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('x'),
                    kind: Press,
                    ..
                }) => {
                    self.disconnect();
                }
                _ => {}
            };
        }
        Ok(())
    }
}
