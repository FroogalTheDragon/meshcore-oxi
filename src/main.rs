mod ble;

use ble::{connect_mc_device, find_mc_devices, DeviceList, Device};

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
    } else {
        eprint!("Failed to find devices!");
    }

    Ok(())
}
