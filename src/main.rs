#![allow(unused)]
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect, Rows},
    style::Stylize,
    symbols::{block, border},
    text::Line,
    widgets::{self, Block, Paragraph, Row, Table, TableState, Widget},
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::{io, sync::Mutex};
use std::{process::Command, sync::atomic};

use crossterm::event::KeyEventKind::Press;
mod scan;
use scan::scan_networks;

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (Ctrl+C) quit | (Ctrl+R) scan for networks ",
    "(Enter) connect to network | (↑) move up | (↓) move down",
];

#[derive(Debug)]
struct WifiNetwork {
    in_use: bool,
    ssid: String,
    security: String,
}

#[derive(Debug, Default)]
struct App {
    ssid: String,
    password: String,
    connected: bool,
    ip: String,
    error: Arc<Mutex<String>>,
    loading: bool,
    show_password: bool,
    wifi_list: Arc<Mutex<Vec<WifiNetwork>>>,
    exit: bool,
}

impl App {
    fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<(), Box<dyn std::error::Error>> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
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
                    self.connect();
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
                _ => {}
            };
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn connect(&mut self) {
        todo!("logic to connect to the selected network");
    }

    fn update_selected_network(&mut self, direction: isize) {
        todo!("logic to update the selected network based on user input");
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

        let mut header = Vec::new();
        header.push(
            Row::new(vec!["SSID", "SECURITY"]).style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .bold(),
            ),
        );
        match self.wifi_list.try_lock() {
            Ok(wifi_list) => {
                for network in wifi_list.iter() {
                    let sssid = if network.in_use {
                        format!("* {}", network.ssid)
                    } else {
                        network.ssid.clone()
                    };
                    header.push(Row::new(vec![sssid, network.security.clone()]));
                }
            }
            Err(_) => {
                header.push(Row::new(vec!["Scanning...", ""]));
            }
        }

        let widths = [Constraint::Percentage(100), Constraint::Percentage(100)];

        let table = Table::new(header, widths)
            .widths(widths)
            .block(block)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        let mut table_state = TableState::default();
        *table_state.offset_mut() = 1;

        table.render(area, buf);
    }
}

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::try_restore();
    app_result
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tui().unwrap();
    Ok(())
}
