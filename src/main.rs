mod ble;
mod tui;
mod app;
mod contacts;
mod theme;


use std::io::stdout;
use ble::{connect_mc_device, find_mc_devices, DeviceList, Device};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use color_eyre::Result;
use crossterm::style::Color::Blue;
use crossterm::execute;
use ratatui::{TerminalOptions, Viewport, layout::Rect};
use tui::{BleConnectionScreen, Screens};

use futures::StreamExt;
use meshcore_rs::MeshCore;
use std::time::Duration;
use crate::app::App;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    // this size is to match the size of the terminal when running the demo
    // using vhs in a 1280x640 sized window (github social preview size)
    let viewport = Viewport::Fixed(Rect::new(0, 0, 81, 18));
    let mut terminal = ratatui::init_with_options(TerminalOptions { viewport });
    execute!(stdout(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let app_result = App::default().run(&mut terminal);
    execute!(stdout(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();
    app_result
}
