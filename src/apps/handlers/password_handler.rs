use super::WifiInputState;
use super::utils::{delete_char, enter_char, move_cursor_right};
use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::io;
use std::time::Duration;

impl WifiInputState {
    pub fn handle_password_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))?
            && let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
        {
            match (code, modifiers) {
                (KeyCode::Left, _) => {
                    self.move_cursor_left();
                }
                (KeyCode::Right, _) => {
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                (KeyCode::Esc, _) => {
                    // if we go back from password input, we should show the ssid popup again
                    // with the cursor at the end of the ssid
                    self.flags.show_password_popup = false;
                    if self.flags.is_hidden {
                        self.flags.show_ssid_popup = true;
                        self.cursor_pos = self.ssid.chars().count() as u16;
                    }
                }
                (KeyCode::Char(c), _) => {
                    enter_char(&mut self.password, c, &self.cursor_pos);
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                (KeyCode::Backspace, _) => {
                    delete_char(&mut self.password, &mut self.cursor_pos);
                    self.move_cursor_left()
                }
                (KeyCode::Enter, _) => {
                    if self.password.is_empty() || self.password.chars().count() >= 8 {
                        self.prepare_to_connect();
                    }
                }
                _ => {}
            }
        }
        Ok(())
    }
}
