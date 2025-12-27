#![allow(unused)]
mod event_handlers;
mod run;
mod widget;
use crate::AppState;
use crate::WifiCredentials;
use crate::WifiNetwork;
use crate::scan_networks;
use crossterm::cursor::DisableBlinking;
use crossterm::cursor::EnableBlinking;
use crossterm::cursor::{self, MoveTo};
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers, poll};
use crossterm::execute;
use ratatui::widgets::Clear;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
};
use std::io;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (Ctrl+C) quit | (Ctrl+R) scan for networks ",
    "(Enter|o) connect to network | (↑|k) move up | (↓|j) move down",
];

#[derive(Debug)]
pub struct App {
    wifi_credentials: WifiCredentials,
    wifi_list: Arc<Mutex<Vec<WifiNetwork>>>,
    selected: usize,
    app_state: AppState,
}

impl Default for App {
    fn default() -> Self {
        let wifi_list = Arc::new(Mutex::new(Vec::new()));
        scan_networks(wifi_list.clone());
        Self {
            wifi_credentials: WifiCredentials::default(),
            wifi_list,
            selected: 0,
            app_state: AppState::default(),
        }
    }
}

impl App {
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn exit(&mut self) {
        self.app_state.exit = true;
    }

    fn prepare_to_connect(&mut self) {
        match self.wifi_list.try_lock() {
            Ok(wifi_list) => {
                if wifi_list[self.selected].in_use {
                } else if wifi_list[self.selected].security == "--" {
                    self.wifi_credentials.ssid = wifi_list[self.selected].ssid.clone();
                    self.wifi_credentials.password.clear();
                } else if wifi_list[self.selected].ssid == "Connect to Hidden network" {
                    self.wifi_credentials.flags.is_hidden = true;
                    self.wifi_credentials.flags.show_ssid_popup = true;

                    // if the wifi is hidden, then the ssid should be entered manually and the
                    // passoword popupo should be shown by the listner of the enter of the in the
                    // ssid input
                    self.wifi_credentials.flags.show_password_popup = false;
                    self.wifi_credentials.ssid.clear();
                    self.wifi_credentials.password.clear();
                } else {
                    self.wifi_credentials.flags.show_password_popup = true;
                    self.wifi_credentials.ssid = wifi_list[self.selected].ssid.clone();
                    self.wifi_credentials.password.clear();
                }
            }
            Err(_) => {
                panic!("Failed to acquire lock on wifi_list");
            }
        }
    }

    fn update_selected_network(&mut self, direction: isize) {
        if let Ok(wifi_list) = self.wifi_list.try_lock() {
            let len = wifi_list.len();
            if len > 0 {
                self.selected =
                    // Handle wrapping around the list
                    ((self.selected as isize + direction).rem_euclid(len as isize)) as usize;
            }
        }
    }
}
