use crate::connect_to_network;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind::Press, KeyModifiers, poll};
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
            status_message: status_message,
            status_code: ExitStatus::default(),
        }
    }
}

impl Status {
    pub fn new(status_message: String, status_code: ExitStatus) -> Self {
        Self {
            status_message: status_message,
            status_code: ExitStatus::default(),
        }
    }
}

#[derive(Debug)]
pub struct WifiCredentials {
    pub is_hidden: bool,
    pub ssid: String,
    pub password: String,
    pub cursor_pos: u16,
    pub show_password_popup: bool,
    pub show_ssid_popup: bool,
    pub status: Status,
}

impl Default for WifiCredentials {
    fn default() -> Self {
        WifiCredentials {
            is_hidden: false,
            ssid: String::new(),
            password: String::new(),
            cursor_pos: 0,
            show_password_popup: false,
            show_ssid_popup: false,
            status: Status::default(),
        }
    }
}
impl WifiCredentials {
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
                    self.show_ssid_popup = false;
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
                    self.show_ssid_popup = false;
                    self.show_password_popup = true;
                    self.cursor_pos = self.password.chars().count() as u16;
                }
                _ => {}
            };
        }
        Ok(())
    }

    pub fn handle_password_input(&mut self) -> io::Result<()> {
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
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    // if we go back from password input, we should show the ssid popup again
                    // with the cursor at the end of the ssid
                    self.show_password_popup = false;
                    self.show_ssid_popup = true;
                    self.cursor_pos = self.ssid.chars().count() as u16;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    enter_char(&mut self.password, c, &self.cursor_pos);
                    move_cursor_right(&self.password, &mut self.cursor_pos);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    delete_char(&mut self.password, &mut self.cursor_pos);
                    self.move_cursor_left()
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.prepare_to_connect();
                }
                _ => {}
            };
        }
        Ok(())
    }

    pub fn handle_status_message(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.status.status_message.clear();
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn prepare_to_connect(&mut self) {
        self.show_password_popup = false;
        self.is_hidden = false;
        self.status = connect_to_network(&self);
        self.reset_cursor_position();
    }

    fn move_cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    fn reset_cursor_position(&mut self) {
        self.cursor_pos = 0;
    }
}

pub fn move_cursor_right(string: &String, cursor_pos: &mut u16) {
    // ensuring the cursor does not go beyond the string length
    *cursor_pos = cursor_pos.saturating_add(1);
    *cursor_pos = (*cursor_pos).min(string.chars().count() as u16);
}

pub fn delete_char(string: &mut String, cursor_pos: &mut u16) {
    if *cursor_pos > 0 {
        let char_index_to_delete = *cursor_pos as usize - 1;
        // getting all the chars before the char to delete
        let before_char_to_delete = string.chars().take(char_index_to_delete);

        // getting all the chars after the car to delete
        let after_char_to_delete = string.chars().skip(*cursor_pos as usize);

        *string = before_char_to_delete.chain(after_char_to_delete).collect();
    }
}

pub fn enter_char(string: &mut String, c: char, cursor_pos: &u16) {
    let index = byte_index(string, cursor_pos);
    string.insert(index, c);
}

// getting the byte index of the cursor position in the string(utf-8)
pub fn byte_index(string: &String, cursor_pos: &u16) -> usize {
    string
        .char_indices()
        .map(|(i, _)| i)
        .nth(*cursor_pos as usize)
        .unwrap_or(string.len())
}
