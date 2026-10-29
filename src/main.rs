mod ble;
mod tui;
mod app;
mod contacts;

use ble::{connect_mc_device, find_mc_devices, DeviceList, Device};
use crossterm::style::Color::Blue;
use tui::{BleConnectionScreen, Screens};

use futures::StreamExt;
use meshcore_rs::MeshCore;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let devices: Option<DeviceList> = find_mc_devices(5, true).await?;

    println!("beans");
    if let Some(d) = devices {
        println!("Attempting to connect to {:?}", d.devices.first().unwrap());
        let conn = connect_mc_device(d.devices.first().unwrap()).await?;
        let screen = Screens::default();
        dbg!(screen);
    } else {
        eprint!("Failed to find devices!");
    }

    Ok(())
}
