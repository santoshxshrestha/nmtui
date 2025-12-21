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

use crossterm::event::KeyEventKind::Press;
#[derive(Debug, Default)]
struct App {
    search: bool,
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
            _ => {}
        };
        Ok(())
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("NMTUI")
            .title_alignment(ratatui::layout::Alignment::Center);
        block.render(area, buf);
        let content = Paragraph::new(Line::from(format!("{:#?}", self)));
        content.render(
            Rect {
                x: area.x + 1,
                y: area.y + 1,
                width: area.width - 2,
                height: area.height - 2,
            },
            buf,
        );
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
