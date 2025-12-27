#![allow(unused)]
use super::WifiInputState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind::Press, poll};
use std::io;
use std::process::ExitStatus;
use std::time::Duration;
impl WifiInputState {
    pub fn handle_ssid_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Left,
                    kind: Press,
                    ..
                }) => {
                    self.move_cursor_left();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Right,
                    kind: Press,
                    ..
                }) => {
                    move_cursor_right(&self.ssid, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.flags.show_ssid_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    enter_char(&mut self.ssid, c, &self.cursor_pos);
                    move_cursor_right(&self.ssid, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    delete_char(&mut self.ssid, &mut self.cursor_pos);
                    self.move_cursor_left();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    // when ssid is entered, we should show the password popup
                    // but if the user had entered a password before, we should keep it
                    // so that the user can go back and forth without losing the password
                    self.flags.show_ssid_popup = false;
                    self.flags.show_password_popup = true;
                    self.cursor_pos = self.password.chars().count() as u16;
                }
                _ => {}
            };
        }
        Ok(())
    }
}
