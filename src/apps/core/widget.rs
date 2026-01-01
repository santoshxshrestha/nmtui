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
    "[Esc] quit | (Ctrl+R) scan for networks | (h) help ",
    "(Enter) connect to network | (↑) move up | (↓) move down",
];

const HELP_TEXT: [&str; 12] = [
    "[Esc] quit",
    "(Ctrl+c) force quit",
    "(Ctrl+R) scan for networks",
    "(Enter) connect to network",
    "(o) connect to network",
    "(d) delete saved network",
    "(↑|k) move up",
    "(↓|j) move down",
    "(h) help",
    "(?) help",
    "(s) view saved networks",
    "(x) disconnect from current network",
];

impl Widget for &App {
    /// Render the application's terminal UI into the provided drawing area buffer.
    ///
    /// This draws the main network table and, depending on internal flags and state,
    /// overlays the saved-connections list, help menu, delete-confirmation popup,
    /// hidden-SSID input popup, password input popup, and status popup. Cursor
    /// visibility and blinking are adjusted as needed for input popups.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use ratatui::buffer::Buffer;
    /// use ratatui::layout::Rect;
    ///
    /// let app = App::new(); // construct your App
    /// let mut buf = Buffer::empty(Rect::new(0, 0, 80, 24));
    /// app.render(Rect::new(0, 0, 80, 24), &mut buf);
    /// ```
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from("NMTUI").bold().italic().centered();

        let block = Block::default()
            .borders(ratatui::widgets::Borders::ALL)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Blue))
            .border_type(ratatui::widgets::BorderType::Rounded)
            .title(title)
            .title_bottom(Line::from(INFO_TEXT.join(" ")).italic().centered());

        let header = Row::new(vec!["SSID", "SECURITY", "SAVED"]).style(
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
                    let mut row = Row::new(vec![
                        ssid,
                        network.security.clone(),
                        network.is_saved.to_string(),
                    ]);
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

        let widths = [
            Constraint::Percentage(60),
            Constraint::Percentage(20),
            Constraint::Percentage(20),
        ];

        let table = Table::new(rows, widths)
            .header(header)
            .block(block)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

        let mut table_state = TableState::default();
        table_state.select(Some(self.selected));

        table.render(area, buf);
        // handle the render of the saved connections list
        if self.flags.show_saved {
            Clear.render(area, buf);
            let _ = execute!(io::stdout(), cursor::Hide, DisableBlinking);
            let saved_block = Block::default()
                .title("Saved Connections")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta));

            let saved_area = Rect {
                x: area.x + area.width / 6,
                y: area.y + area.height / 6,
                width: area.width * 2 / 3,
                height: area.height * 2 / 3,
            };

            let mut saved_rows = Vec::new();
            for (i, connection) in self.saved_connection.connections.iter().enumerate() {
                let mut row = Row::new(vec![connection.ssid.clone(), connection.last_used.clone()]);
                if i == self.saved_connection.selected_index {
                    row = row.style(
                        ratatui::style::Style::default()
                            .fg(ratatui::style::Color::Black)
                            .bg(ratatui::style::Color::White),
                    );
                }
                saved_rows.push(row);
            }

            let saved_header = Row::new(vec!["SSID", "LAST USED"]).style(
                ratatui::style::Style::default()
                    .fg(ratatui::style::Color::Yellow)
                    .bold(),
            );

            let saved_table = Table::new(
                saved_rows,
                [Constraint::Percentage(70), Constraint::Percentage(30)],
            )
            .header(saved_header)
            .block(saved_block)
            .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let mut saved_table_state = TableState::default();
            saved_table_state.select(Some(self.saved_connection.selected_index));

            saved_table.render(saved_area, buf);
        }

        // handle the render of the help menu
        if self.flags.show_help {
            Clear.render(area, buf);
            let help_block = Block::default()
                .title("Help Menu")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta));

            let help_area = Rect {
                x: area.x + area.width / 6,
                y: area.y + area.height / 6,
                width: area.width * 2 / 3,
                height: area.height * 2 / 3,
            };

            let help_paragraph = Paragraph::new(HELP_TEXT.join("\n"))
                .block(help_block)
                .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let _ = execute!(io::stdout(), cursor::Hide, DisableBlinking);

            help_paragraph.render(help_area, buf);
        }

        // handle the render of the delete confirmation popup
        if self.flags.show_delete_confirmation {
            Clear.render(area, buf);
            let popup_block = Block::default()
                .title("Confirm Deletion")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
                .title_bottom("Are you sure you want to delete this saved network? (y/n)");

            let popup_area = Rect {
                x: area.x + area.width / 4,
                y: area.y + area.height / 4,
                width: area.width / 2,
                height: area.height / 10,
            };

            let confirmation_paragraph =
                Paragraph::new("Press 'y' to confirm deletion or 'n' to cancel.")
                    .block(popup_block)
                    .style(ratatui::style::Style::default().fg(ratatui::style::Color::White));

            let _ = execute!(io::stdout(), cursor::Hide, DisableBlinking);

            confirmation_paragraph.render(popup_area, buf);
        }

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

        // handle the render of the password input popup
        if self.wifi_credentials.flags.show_password_popup {
            Clear.render(area, buf);
            let popup_block = Block::default()
                .title("Enter Password")
                .borders(ratatui::widgets::Borders::ALL)
                .border_type(ratatui::widgets::BorderType::Rounded)
                .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::Magenta))
                .title_bottom("If the network is open, just press Enter");

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

        // handle the render of the status popup
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
