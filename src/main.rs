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
    fn handle_events(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        todo!(" write an event handling logic here ")
    }
}

impl Widget for App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title("NMTUI")
            .title_alignment(ratatui::layout::Alignment::Center);
        block.render(area, buf);
    }
}

fn main() {
    println!("Hello, world!");
}
