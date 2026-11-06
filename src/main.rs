mod app;
mod ble;
mod contacts;
mod db;
mod error;
mod theme;
mod tui;


use ble::{Device, DeviceList};
use color_eyre::Result;
use crossterm::execute;
use crossterm::style::Color::Blue;
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use ratatui::{TerminalOptions, Viewport, layout::Rect};
use std::io::stdout;
use tui::{BleConnectionScreen, Screens};

use crate::app::App;
use futures::StreamExt;
use meshcore_rs::MeshCore;
use std::time::Duration;

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
