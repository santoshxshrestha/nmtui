use super::App;
use ratatui::DefaultTerminal;

impl App {
    pub fn run(
        &mut self,
        terminal: &mut DefaultTerminal,
    ) -> Result<(), Box<dyn std::error::Error>> {
        while !self.app_state.exit {
            terminal.draw(|frame| self.draw(frame))?;
            // NOTE: here placement is is key because we want to prioritize which popup gets handled first
            // If  there is a preceeding popup shown, we want to handle the one that is on top
            // priority with out handling the events of the other one

            // to handle the help popup
            if self.show_help {
                self.handle_help()?;
            }
            // to handle the delete confirmation popup
            else if self.show_delete_confirmation {
                self.handle_delete_confirmation()?;
            }
            // to handle the saved connections popup
            else if self.show_saved {
                self.handle_saved()?;
            }
            // to handle the wifi ssid input popups
            else if self.wifi_credentials.flags.show_ssid_popup {
                self.wifi_credentials.handle_ssid_input()?;
            }
            //to handle the wifi password input popups
            else if self.wifi_credentials.flags.show_password_popup {
                self.wifi_credentials.handle_password_input()?;
            }
            // to handle the status message popup
            else if self.wifi_credentials.flags.show_status_popup {
                self.wifi_credentials.handle_status_message()?;
            }
            // to handle the main events
            else {
                self.handle_events()?;
            }
        }
        Ok(())
    }
}
