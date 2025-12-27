#![allow(unused)]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind::Press, poll};
use std::io;
use std::process::ExitStatus;
use std::time::Duration;

#[derive(Debug, Default)]
pub struct Flags {
    pub is_hidden: bool,
    pub show_password_popup: bool,
    pub show_ssid_popup: bool,
    pub show_status_popup: bool,
}
