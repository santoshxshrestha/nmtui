use super::App;
use ratatui::DefaultTerminal;

impl App {
    /// Run the application's main event loop until the app requests exit.
    ///
    /// Renders the UI each iteration and dispatches input to the highest-priority active popup or, when no popup is active, to the main event handler. The loop continues until `self.app_state.exit` is true or an error occurs.
    ///
    /// # Returns
    ///
    /// `Ok(())` on normal termination, or an error if a rendering or handler callback fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // assume `App` and `DefaultTerminal` are available and constructible
    /// let mut app = App::default();
    /// let mut terminal = DefaultTerminal::new();
    /// app.run(&mut terminal).unwrap();
    /// ```
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
            if self.flags.show_help {
                self.handle_help()?;
            }
            // to handle the delete confirmation popup this is at top becuase it is displayed
            // over other popups
            else if self.flags.show_delete_confirmation {
                self.handle_delete_confirmation()?;
            }
            // to handle the saved connections popup
            else if self.flags.show_saved {
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
