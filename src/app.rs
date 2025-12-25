#![allow(unused)]
use crate::AppState;
use crate::WifiCredentials;
use crate::WifiNetwork;
use crate::connect_to_network;
use crate::scan;
use crate::scan_networks;
use crossterm::ExecutableCommand;
use crossterm::cursor::EnableBlinking;
use crossterm::cursor::{self, MoveTo};
use crossterm::event::KeyEventKind::Press;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll};
use crossterm::execute;
use ratatui::widgets::Clear;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect, Rows},
    style::Stylize,
    symbols::{block, border},
    text::Line,
    widgets::{self, Block, Paragraph, Row, Table, TableState, Widget},
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
    connected: bool,
    ip: String,
    error: Arc<Mutex<String>>,
    loading: bool,
    show_password_popup: bool,
    show_ssid_popup: bool,
    wifi_list: Arc<Mutex<Vec<WifiNetwork>>>,
    selected: usize,
    app_state: AppState,
}

impl Default for App {
    fn default() -> Self {
        let wifi_list = Arc::new(Mutex::new(Vec::new()));
        scan_networks(wifi_list.clone());
        Self {
            wifi_credentials: WifiCredentials {
                is_hidden: false,
                ssid: String::new(),
                password: String::new(),
                cursor_pos: 0,
            },
            connected: false,
            ip: String::new(),
            error: Arc::new(Mutex::new(String::new())),
            loading: false,
            show_password_popup: false,
            show_ssid_popup: false,
            wifi_list: wifi_list,
            selected: 0,
            app_state: AppState { exit: false },
        }
    }
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.app_state.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if self.show_ssid_popup {
                self.handle_ssid_input()?;
            } else if self.show_password_popup {
                self.handle_password_input()?;
            } else {
                self.handle_events()?;
            }
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

    fn handle_ssid_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.show_ssid_popup = false;
                    self.show_password_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to insert the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.wifi_credentials.ssid.push(c);
                    self.move_cursor_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to delete the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.wifi_credentials.ssid.pop();
                    self.move_cursor_left();
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
    fn handle_password_input(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.show_password_popup = false;
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }

                Event::Key(KeyEvent {
                    code: KeyCode::Char(c),
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to insert the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.wifi_credentials.password.push(c);
                    self.move_cursor_right();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Backspace,
                    kind: Press,
                    ..
                }) => {
                    // TODO: change the logic to pop the content in the current curser position
                    // by giving the user to move the cursor to the left and right if they have done some mistake
                    self.wifi_credentials.password.pop();
                    self.move_cursor_left()
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.show_password_popup = false;
                    connect_to_network(&self.wifi_credentials);
                    self.reset_cursor_position();
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if poll(Duration::from_micros(1))? {
            match event::read()? {
                Event::Key(KeyEvent {
                    code: KeyCode::Esc,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('c'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    self.exit();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('r'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: Press,
                    ..
                }) => {
                    scan_networks(self.wifi_list.clone());
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    kind: Press,
                    ..
                }) => {
                    self.prepare_to_connect();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('o'),
                    kind: Press,
                    ..
                }) => {
                    self.prepare_to_connect();
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Up,
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(-1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Down,
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(1);
                }
                // vim style
                Event::Key(KeyEvent {
                    code: KeyCode::Char('k'),
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(-1);
                }
                Event::Key(KeyEvent {
                    code: KeyCode::Char('j'),
                    kind: Press,
                    ..
                }) => {
                    self.update_selected_network(1);
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.app_state.exit = true;
    }

    fn prepare_to_connect(&mut self) {
        match self.wifi_list.try_lock() {
            Ok(wifi_list) => {
                if wifi_list[self.selected].in_use {
                    return;
                } else if wifi_list[self.selected].security == "--" {
                    self.wifi_credentials.ssid = wifi_list[self.selected].ssid.clone();
                    self.wifi_credentials.password.clear();
                } else if wifi_list[self.selected].ssid == "Connect to Hidden network" {
                    self.wifi_credentials.is_hidden = true;
                    self.show_ssid_popup = true;

                    // if the wifi is hidden, then the ssid should be entered manually and the
                    // passoword popupo should be shown by the listner of the enter of the in the
                    // ssid input
                    self.show_password_popup = false;
                    self.wifi_credentials.ssid.clear();
                    self.wifi_credentials.password.clear();
                } else {
                    self.show_password_popup = true;
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
    fn move_cursor_left(&mut self) {
        self.wifi_credentials.cursor_pos = self.wifi_credentials.cursor_pos.saturating_sub(1);
    }

    fn move_cursor_right(&mut self) {
        self.wifi_credentials.cursor_pos = self.wifi_credentials.cursor_pos.saturating_add(1);
    }
    fn reset_cursor_position(&mut self) {
        self.wifi_credentials.cursor_pos = 0;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("NMTUI").bold().italic().centered();

        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(title)
            .title_bottom(Line::from(INFO_TEXT.join(" ")).italic().centered());

        let header = Row::new(vec!["SSID", "SECURITY"]).style(
            ratatui::style::Style::default()
                .fg(ratatui::style::Color::Yellow)
                .bold(),
        );

        let mut rows = Vec::new();
        match self.wifi_list.try_lock() {
            Ok(wifi_list) => {
                for (i, network) in wifi_list.iter().enumerate() {
                    let ssid = if network.in_use {
                        format!("* {}", network.ssid)
                    } else {
                        network.ssid.clone()
                    };
                    let mut row = Row::new(vec![ssid, network.security.clone()]);
                    if i == self.selected {
                        row = row.style(
                            ratatui::style::Style::default()
                                .fg(ratatui::style::Color::Black)
                                .bg(ratatui::style::Color::White),
                        );
                    }
                    rows.push(row);
                }
            }
            Err(_) => {
                rows.push(Row::new(vec!["Scanning...", ""]));
            }
        }

        let widths = [Constraint::Percentage(70), Constraint::Percentage(30)];

        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        let mut table_state = TableState::default();
        table_state.select(Some(self.selected));

        table.render(area, buf);

        // handle the render of the ssid input popup for hidden networks
        if self.wifi_credentials.is_hidden {
            Clear.render(area, buf);
            let popup_block = Block::default()
                .title("Enter the ssid of the hidden network")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta));

            let popup_area = Rect {
                x: area.x + area.width / 4,
                y: area.y + area.height / 4,
                width: area.width / 2,
                height: area.height / 4,
            };

            let ssid_paragraph = Paragraph::new(self.wifi_credentials.ssid.as_str())
                .block(popup_block)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let _ = execute!(
                io::stdout(),
                cursor::Show,
                MoveTo(
                    popup_area.x + self.wifi_credentials.cursor_pos + 1,
                    popup_area.y + 1,
                ),
                EnableBlinking
            );

            ssid_paragraph.render(popup_area, buf);
        }

        if self.show_password_popup {
            Clear.render(area, buf);
            let popup_block = Block::default()
                .title("Enter Password")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
                .title_bottom("If the network is open | already saved, just press Enter");

            let popup_area = Rect {
                x: area.x + area.width / 4,
                y: area.y + area.height / 4,
                width: area.width / 2,
                height: area.height / 4,
            };

            let password_paragraph = Paragraph::new(self.wifi_credentials.password.as_str())
                .block(popup_block)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let _ = execute!(
                io::stdout(),
                cursor::Show,
                MoveTo(
                    popup_area.x + self.wifi_credentials.cursor_pos + 1,
                    popup_area.y + 1,
                ),
                EnableBlinking
            );

            password_paragraph.render(popup_area, buf);
        }
    }
}
