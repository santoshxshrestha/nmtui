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
use ratatui::Frame;
use std::sync::{Arc, Mutex};
mod delete_handler;
mod help_handlers;

#[derive(Debug)]
pub struct App {
    wifi_credentials: WifiInputState,
    wifi_list: Arc<Mutex<Vec<WifiNetwork>>>,
    selected: usize,
    app_state: AppState,
    saved_connection: SavedConnections,
    flags: Flags,
}

impl Default for App {
    fn default() -> Self {
        let wifi_list = Arc::new(Mutex::new(Vec::new()));
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
    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }

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
}
