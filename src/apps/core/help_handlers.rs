use super::App;

use crossterm::event::poll;
use crossterm::event::{self, Event, KeyEvent};
use std::{io, time::Duration};

impl App {
    pub fn handle_help(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: event::KeyCode::Esc,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Enter,
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('q'),
                    kind: event::KeyEventKind::Press,
                    ..
                }) => {
                    self.close_help();
                }
                Event::Key(KeyEvent {
                    code: event::KeyCode::Char('c'),
                    kind: event::KeyEventKind::Press,
                    modifiers: event::KeyModifiers::CONTROL,
                    ..
                }) => {
                    self.exit();
                }
                _ => {}
            };
        }
        Ok(())
    }
    pub fn close_help(&mut self) {
        self.show_help = false;
    }
}
