use super::utils::{delete_char, enter_char, move_cursor_right};
use crate::apps::handlers::WifiInputState;
use crossterm::event::{self, Event, KeyCode, KeyEvent, poll};
use std::io;
use std::time::Duration;

impl WifiInputState {
    pub fn handle_ssid_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))?
            && let Event::Key(KeyEvent { code, .. }) = event::read()?
        {
            match code {
                KeyCode::Esc => {
                    self.flags.show_ssid_popup = false;
                    return Ok(());
                }
                KeyCode::Char(c) => {
                    enter_char(&mut self.ssid, c, &self.cursor_pos);
                    move_cursor_right(&self.ssid, &mut self.cursor_pos);
                    return Ok(());
                }
                KeyCode::Backspace => {
                    delete_char(&mut self.ssid, &mut self.cursor_pos);
                    self.move_cursor_left();
                    return Ok(());
                }
                KeyCode::Enter => {
                    // when ssid is entered, we should show the password popup
                    // but if the user had entered a password before, we should keep it
                    // so that the user can go back and forth without losing the password
                    self.flags.show_ssid_popup = false;
                    self.flags.show_password_popup = true;
                    self.cursor_pos = self.password.chars().count() as u16;
                    return Ok(());
                }
                KeyCode::Left => {
                    self.move_cursor_left();
                }
                KeyCode::Right => {
                    move_cursor_right(&self.ssid, &mut self.cursor_pos);
                }
                _ => {}
            }
        }
        Ok(())
    }
}
