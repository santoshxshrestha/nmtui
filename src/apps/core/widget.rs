use super::App;
use crossterm::cursor::DisableBlinking;
use crossterm::cursor::EnableBlinking;
use crossterm::cursor::{self, MoveTo};
use crossterm::execute;
use ratatui::widgets::Clear;
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    text::Line,
    widgets::{Block, Paragraph, Row, Table, TableState, Widget},
};
use std::io;

const INFO_TEXT: [&str; 2] = [
    "(Esc) quit | (Ctrl+C) quit | (Ctrl+R) scan for networks ",
    "(Enter|o) connect to network | (↑|k) move up | (↓|j) move down",
];

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
        if self.wifi_credentials.flags.is_hidden {
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
                height: area.height / 10,
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

        if self.wifi_credentials.flags.show_password_popup {
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
                height: area.height / 10,
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

        if self.wifi_credentials.flags.show_status_popup {
            Clear.render(area, buf);
            let status_block = Block::default()
                .title("Status")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta));

            let status_area = Rect {
                x: area.x + area.width / 4,
                y: area.y + area.height / 3,
                width: area.width / 2,
                height: area.height / 4,
            };
            let stauts = format!(
                "{}\n{}",
                self.wifi_credentials.status.status_code,
                self.wifi_credentials.status.status_message
            );

            let status_paragraph = Paragraph::new(stauts)
                .block(status_block)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let _ = execute!(io::stdout(), cursor::Hide, DisableBlinking);

            status_paragraph.render(status_area, buf);
        }
    }
}
