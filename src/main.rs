use color_eyre::eyre::Result;
mod app;
mod scan;
mod utils;
use scan::scan_networks;
use utils::connect_to_network;
use utils::tui;
mod creadentials;
use creadentials::WifiCredentials;

#[derive(Debug)]
struct WifiNetwork {
    in_use: bool,
    ssid: String,
    security: String,
}

#[derive(Default, Debug)]
struct AppState {
    exit: bool,
}

fn main() -> Result<()> {
    color_eyre::install()?;
    tui().unwrap();
    Ok(())
}
