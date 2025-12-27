use super::WifiInputState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind::Press, poll};
use std::io;
use std::process::ExitStatus;
use std::time::Duration;

#[derive(Debug)]
pub struct Status {
    pub status_message: String,
    pub status_code: ExitStatus,
}
impl Default for Status {
    fn default() -> Self {
        let status_message = String::new();
        Status {
            status_message,
            status_code: ExitStatus::default(),
        }
    }
}

impl Status {
    pub fn new(status_message: String, status_code: ExitStatus) -> Self {
        Self {
            status_message,
            status_code,
        }
    }
}

impl WifiInputState {
    pub fn handle_status_message(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_status_popup = false;
                    self.status.status_message.clear();
                    self.status.status_code = ExitStatus::default();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_status_popup = false;
                    self.status.status_message.clear();
                    self.status.status_code = ExitStatus::default();
                }
                _ => {}
            }
        };
        Ok(())
    }
}
