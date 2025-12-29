use super::App;
use crate::utils::delete_connection::delete_connection;
use crate::utils::scan::scan_networks;

use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::io;
use std::time::Duration;

impl App {
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
                    self.show_delete_confirmation = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('n'),
                    kind: Press,
                    ..
                }) => {
                    self.show_delete_confirmation = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.show_delete_confirmation = false;
                }

                _ => {}
            }
        };
        Ok(())
    }
    pub fn delete_connection(&mut self) {
        if self.show_saved {
            delete_connection(
                self.saved_connection.connections[self.selected]
                    .ssid
                    .clone(),
            );
        } else {
            delete_connection(self.wifi_list.lock().unwrap()[self.selected].ssid.clone());
        }
        self.show_delete_confirmation = false;
        scan_networks(self.wifi_list.clone());
    }
}
