use super::WifiInputState;
use super::utils::{delete_char, enter_char, move_cursor_right};
use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::io;
use std::time::Duration;

impl WifiInputState {
    pub fn handle_password_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    ..
                }) => {
                    self.move_cursor_left();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    ..
                }) => {
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc, ..
                }) => {
                    // if we go back from password input, we should show the ssid popup again
                    // with the cursor at the end of the ssid
                    self.flags.show_password_popup = false;
                    if self.flags.is_hidden {
                        self.flags.show_ssid_popup = true;
                        self.cursor_pos = self.ssid.chars().count() as u16;
                    }
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    ..
                }) => {
                    enter_char(&mut self.password, c, &self.cursor_pos);
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    ..
                }) => {
                    delete_char(&mut self.password, &mut self.cursor_pos);
                    self.move_cursor_left()
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    if self.password.is_empty() || self.password.chars().count() >= 8 {
                        self.prepare_to_connect();
                    }
                }
                _ => {}
            };
        }
        Ok(())
    }
}
