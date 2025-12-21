#![allow(unused)]
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use std::io;
use std::time::Duration;

use crossterm::event::KeyEventKind::Press;
#[derive(Debug, Default)]
struct App {
    is_scanning: bool,
    ssid: String,
    password: String,
    connected: bool,
    ip: String,
    error: String,
    loading: bool,
    show_password: bool,
    wifi_list: Vec<String>,
    exit: bool,
}

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
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
                    self.scan_networks();
                }
                _ => {}
            };
        }
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn scan_networks(&mut self) {
        self.is_scanning = true;
        // Simulate scanning
        std::thread::sleep(std::time::Duration::from_secs(2));
        self.wifi_list = vec![
            "Network_1".to_string(),
            "Network_2".to_string(),
            "Network_3".to_string(),
        ];
        self.is_scanning = false;
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
        block.render(area, buf);

        let content_area = Rect {
            x: area.x + 1,
            y: area.y + 2,
            width: area.width - 2,
            height: area.height - 3,
        };

        let content = if self.is_scanning {
            "Scanning for networks...".to_string()
        } else if !self.wifi_list.is_empty() {
            format!("Available Networks:\n{}", self.wifi_list.join("\n"))
        } else {
            "No networks found. Press Ctrl+R to scan.".to_string()
        };

        let content_paragraph = Paragraph::new(Line::from(content));
        content_paragraph.render(content_area, buf);

        // let content = Paragraph::new(Line::from(format!("{:#?}", self)));
        // content.render(
        //     Rect {
        //         x: area.x + 1,
        //         y: area.y + 1,
        //         width: area.width - 2,
        //         height: area.height - 2,
        //     },
        //     buf,
        // );
    }
}

pub fn tui() -> Result<(), Box<dyn std::error::Error>> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::try_restore();
    app_result
}

fn main() {
    tui().unwrap()
}
