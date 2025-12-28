use super::App;
use ratatui::DefaultTerminal;

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.app_state.exit {
            terminal.draw(|frame| self.draw(frame))?;
            if self.show_help {
                self.handle_help()?;
            } else if self.show_delete_confirmation {
                self.handle_delete_confirmation()?;
            } else if self.wifi_credentials.flags.show_ssid_popup {
                self.wifi_credentials.handle_ssid_input()?;
            } else if self.wifi_credentials.flags.show_password_popup {
                self.wifi_credentials.handle_password_input()?;
            } else if self.wifi_credentials.flags.show_status_popup {
                self.wifi_credentials.handle_status_message()?;
            } else {
                self.handle_events()?;
            }
        }
        Ok(())
    }
}
