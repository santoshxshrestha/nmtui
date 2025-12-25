use crate::connect_to_network;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind::Press, KeyModifiers, poll};
use std::io;
use std::time::Duration;

#[derive(Debug)]
pub struct WifiCredentials {
    pub is_hidden: bool,
    pub ssid: String,
    pub password: String,
    pub cursor_pos: u16,
    pub show_password_popup: bool,
    pub show_ssid_popup: bool,
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
                    self.move_cursor_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.show_ssid_popup = false;
                    self.show_password_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    //enter_char handles the cursor position internally
                    self.enter_char(c);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    // delete_char handles the cursor position internally
                    self.delete_char();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.show_ssid_popup = false;
                    self.show_password_popup = true;
                    self.reset_cursor_position();
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
                    self.move_cursor_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.show_password_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to insert the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.password.push(c);
                    self.move_cursor_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to pop the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.password.pop();
                    self.move_cursor_left()
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.show_password_popup = false;
                    connect_to_network(&self);
                    self.reset_cursor_position();
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn enter_char(&mut self, c: char) {
        let index = self.byte_index();
        self.ssid.insert(index, c);
        self.move_cursor_right();
    }

    // Todo: need to refactore this function with the acutal data type
    // here we are doing like this because removing a char from a string in rust is not straightforward
    fn delete_char(&mut self) {
        let cursor_pos = self.cursor_pos;
        if cursor_pos > 0 {
            let char_index_to_delete = cursor_pos as usize - 1;
            // getting all the chars before the char to delete
            let before_char_to_delete = self.ssid.chars().take(char_index_to_delete);

            // getting all the chars after the car to delete
            let after_char_to_delete = self.ssid.chars().skip(cursor_pos as usize);

            self.ssid = before_char_to_delete.chain(after_char_to_delete).collect();

            // we are deleting the char to we need to more the cursor to the left
            self.move_cursor_left();
        }
    }

    fn move_cursor_left(&mut self) {
        self.cursor_pos = self.cursor_pos.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        // Todo: this will move the cursor to the right and beyound the lenght of the string so need to handle that after
        // doing some indexing in the string of the ssid and password
        self.cursor_pos = self.cursor_pos.saturating_add(1);
    }
    fn reset_cursor_position(&mut self) {
        self.cursor_pos = 0;
    }

    // getting the byte index of the cursor position in the string(utf-8)
    fn byte_index(&self) -> usize {
        self.ssid
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.cursor_pos as usize)
            .unwrap_or(self.ssid.len())
    }
}
