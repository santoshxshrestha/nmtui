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

fn scan_networks(mut wifi_list: Arc<Mutex<Vec<WifiNetwork>>>, is_scalling: AtomicBool) {
    // self.is_scanning = true;
    // // nmcli -t -f IN-USE,SSID,SECURITY device wifi list

    thread::spawn(move || {
        let output = Command::new("nmcli")
            .arg("-t")
            .arg("-f")
            .arg("IN-USE,SSID,SECURITY")
            .arg("device")
            .arg("wifi")
            .arg("list")
            .output()
            .expect("Failed to execute nmcli command");

        if !output.status.success() {
            return;
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut networks: Vec<WifiNetwork> = Vec::new();

        stdout.lines().map(|line| {
            let mut parts = line.splitn(3, ':');

            let in_use = parts.next() == Some("*");
            let ssid = parts.next().unwrap_or("").to_string();
            let security = parts.next().unwrap_or("--").to_string();
            networks.push(WifiNetwork {
                in_use,
                ssid,
                security,
            })
        });
        let mut wifi_list_lock = wifi_list.lock().unwrap();
        *wifi_list_lock = networks;
        is_scalling.store(false, Ordering::SeqCst);
    });

    // handle.join().unwrap();

    // self.is_scanning = false;
}

#[derive(Debug, Default)]
struct App {
    is_scanning: AtomicBool,
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
                    self.is_scanning.store(true, Ordering::SeqCst);
                    scan_networks(self.wifi_list.clone(), self.is_scanning);
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

        let instructions = Line::from("Press ESC or Ctrl+C to exit")
            .italic()
            .centered();

        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(title)
            .title_bottom(instructions);

        let mut header = Vec::new();
        header.push(
            Row::new(vec!["SSID", "SECURITY"]).style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .bold(),
            ),
        );
        if self.is_scanning.load(Ordering::SeqCst) {
            header.push(Row::new(vec!["Scanning for networks...", ""]));
        } else {
            let wifi_list_arc = Arc::clone(&self.wifi_list);
            for network in wifi_list_arc.lock().unwrap().iter() {
                let sssid = if network.in_use {
                    format!("* {}", network.ssid)
                } else {
                    network.ssid.clone()
                };
                header.push(Row::new(vec![sssid, network.security.clone()]));
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
