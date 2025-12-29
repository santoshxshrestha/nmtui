use super::App;

use crossterm::event::poll;
use crossterm::event::{self, Event, KeyEvent};
use std::{io, time::Duration};

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
                }) => {
                    self.delete_saved_connection();
                }
                _ => {}
            };
        }
        Ok(())
    }
    pub fn close_saved_list(&mut self) {
        self.show_saved = false;
    }
}
