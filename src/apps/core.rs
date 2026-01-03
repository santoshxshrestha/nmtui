mod event_handlers;
mod run;
mod saved_connection;
mod widget;
use crate::AppState;
use crate::WifiNetwork;
use crate::apps::core::saved_connection::SavedConnections;
use crate::apps::handlers::WifiInputState;
use crate::apps::handlers::flags::Flags;
use crate::utils::connect::connect_to_saved_network;
use crate::utils::disconnect_connection::disconnect_connected_network;
use crate::utils::scan::scan_networks;
use crossterm::cursor;
use crossterm::cursor::DisableBlinking;
use crossterm::cursor::EnableBlinking;
use crossterm::execute;
use ratatui::Frame;
use ratatui::layout::Position;
use std::io;
use std::sync::{Arc, Mutex};
mod delete_handler;
mod help_handlers;
use std::sync::RwLock;

#[derive(Debug)]
pub struct App {
    wifi_credentials: WifiInputState,
    wifi_list: Arc<RwLock<Vec<WifiNetwork>>>,
    selected: usize,
    app_state: AppState,
    saved_connection: SavedConnections,
    flags: Flags,
}

impl Default for App {
    /// Constructs a new `App` with default field values and begins an initial Wi‑Fi scan.
    ///
    /// All fields are initialized using their `Default` implementations. The `wifi_list` is created
    /// as an empty, shared, thread-safe vector and an initial network scan is triggered to populate it.
    ///
    /// # Examples
    ///
    /// ```
    /// let app = App::default();
    /// assert_eq!(app.selected, 0);
    /// assert!(app.wifi_list.lock().unwrap().is_empty());
    /// ```
    fn default() -> Self {
        let wifi_list = Arc::new(RwLock::new(Vec::new()));
        scan_networks(wifi_list.clone());
        Self {
            wifi_credentials: WifiInputState::default(),
            wifi_list,
            selected: 0,
            app_state: AppState::default(),
            saved_connection: SavedConnections::default(),
            flags: Flags::default(),
        }
    }
}

impl App {
    /// Render the application UI and manage terminal cursor visibility and position for SSID/password popups.
    ///
    /// When an SSID or password popup is visible, this sets the terminal cursor to the popup's input position
    /// and enables cursor blinking; otherwise it hides the cursor and disables blinking.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Assuming `app` is an initialized `App` and `frame` is a mutable `ratatui::Frame`:
    /// // app.draw(&mut frame);
    /// ```
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());

        // Set cursor position for SSID or Password popups
        if self.wifi_credentials.flags.show_ssid_popup
            || self.wifi_credentials.flags.show_password_popup
        {
            frame.set_cursor_position(Position::new(
                frame.area().x + frame.area().width / 4 + self.wifi_credentials.cursor_pos + 1,
                frame.area().y + frame.area().height / 4 + 1,
            ));
            // The reason for using io::stdout() here is that the crossterm execute! macro needs a writable output target to send
            // terminal commands to. io::stdout() provides a handle to the standard output (the terminal), so the commands (Show, MoveTo, EnableBlinking)
            // are sent to the terminal for immediate effect.
            //
            // If you remove io::stdout(), execute! won’t know where to send the commands, resulting in a compilation error.
            // You must provide a valid output stream (like io::stdout()) for terminal control commands to work.
            // EnableBlinking
            let _ = execute!(io::stdout(), cursor::Show, EnableBlinking);
        } else {
            let _ = execute!(io::stdout(), cursor::Hide, DisableBlinking);
        }
    }

    /// Mark the application to exit by setting its internal exit flag.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut app = App::default();
    /// app.exit();
    /// assert!(app.app_state.exit);
    /// ```
    fn exit(&mut self) {
        self.app_state.exit = true;
    }

    fn prepare_to_connect(&mut self) {
        match self.wifi_list.try_lock() {
            Ok(wifi_list) => {
                // if the selected network is already in use, do nothing
                if wifi_list[self.selected].in_use {
                }
                // if the network is unsecured, connect directly so logic will be similar to saved network
                else if wifi_list[self.selected].security == "Unsecured" {
                    let status = connect_to_saved_network(&wifi_list[self.selected].ssid);
                    self.wifi_credentials.status = status;
                    self.wifi_credentials.flags.show_status_popup = true;

                    // refresh the network list after connection attempt
                    scan_networks(self.wifi_list.clone());
                }
                // if the selected network is hidden network option
                // the show status popup will be handled by the password input listener
                else if wifi_list[self.selected].ssid == "Connect to Hidden network" {
                    self.wifi_credentials.flags.is_hidden = true;
                    self.wifi_credentials.flags.show_ssid_popup = true;
                    // if the wifi is hidden, then the ssid should be entered manually and the
                    // passoword popupo should be shown by the listner of the enter of the in the
                    // ssid input
                    self.wifi_credentials.flags.show_password_popup = false;

                    // clear previous ssid and password
                    self.wifi_credentials.ssid.clear();
                    self.wifi_credentials.password.clear();
                }
                // if the network is saved, connect directly
                else if wifi_list[self.selected].is_saved {
                    let status = connect_to_saved_network(&wifi_list[self.selected].ssid);
                    self.wifi_credentials.status = status;
                    self.wifi_credentials.flags.show_status_popup = true;
                    // refresh the network list after connection attempt
                    scan_networks(self.wifi_list.clone());
                }
                // else show the password popup
                else {
                    self.wifi_credentials.flags.show_password_popup = true;
                    self.wifi_credentials.ssid = wifi_list[self.selected].ssid.clone();
                    self.wifi_credentials.password.clear();
                }
            }
            Err(_) => {
                panic!("Failed to acquire lock on wifi_list");
            }
        }
    }

    fn update_selected_network(&mut self, direction: isize) {
        if let Ok(wifi_list) = self.wifi_list.try_lock() {
            let len = wifi_list.len();
            if len > 0 {
                self.selected =
                    // Handle wrapping around the list
                    ((self.selected as isize + direction).rem_euclid(len as isize)) as usize;
            }
        }
    }

    fn disconnect(&mut self) {
        self.wifi_credentials.status = disconnect_connected_network(self.wifi_list.clone());
        self.wifi_credentials.flags.show_status_popup = true;
        scan_networks(self.wifi_list.clone());
    }

    fn reset_selection(&mut self) {
        self.selected = 0;
    }
}
