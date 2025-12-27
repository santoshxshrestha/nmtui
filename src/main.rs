use color_eyre::eyre::Result;
mod apps;
mod utils;
use utils::tui::tui;

#[derive(Debug)]
struct WifiNetwork {
    is_saved: bool,
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
