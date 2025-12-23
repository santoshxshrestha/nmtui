#![allow(unused)]
use color_eyre::eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, poll};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Rect, Rows},
    style::Stylize,
    symbols::{block, border},
    text::Line,
    widgets::{self, Block, Paragraph, Row, Table, TableState, Widget},
};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use std::{io, sync::Mutex};
use std::{process::Command, sync::atomic};

use crossterm::event::KeyEventKind::Press;
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
