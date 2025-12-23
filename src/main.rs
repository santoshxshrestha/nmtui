#![allow(unused)]
use color_eyre::eyre::Result;
mod app;
mod scan;
mod utils;
use scan::scan_networks;
use utils::connect_to_network;
use utils::tui;

#[derive(Debug)]
struct WifiNetwork {
    in_use: bool,
    ssid: String,
    security: String,
}

#[derive(Debug)]
struct WifiCredentials {
    ssid: String,
    password: String,
    cursor_pos: u16,
}

#[derive(Debug)]
struct AppState {
    exit: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tui().unwrap();
    Ok(())
}
