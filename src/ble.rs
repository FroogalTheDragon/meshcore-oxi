use futures::StreamExt;
use meshcore_rs::MeshCore;
use ratatui::widgets::ListState;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct Device {
    name: String,
    is_connected: bool,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            name: String::default(),
            is_connected: false,
        }
    }
}

#[derive(Debug)]
pub struct DeviceList {
    pub devices: Vec<Device>,
    state: ListState,
}

impl DeviceList {
    pub fn new(devices: Vec<Device>, state: ListState) -> Self {
        Self { devices, state }
    }
}

impl Default for DeviceList {
    fn default() -> Self {
        Self {
            devices: Vec::default(),
            state: ListState::default(),
        }
    }
}

/// Find meshcore devices, set scan length
pub async fn find_mc_devices(
    scan_duration: u64,
    filter: bool,
) -> Result<Option<DeviceList>, meshcore_rs::error::Error> {
    let devices = MeshCore::ble_discover(Duration::from_secs(scan_duration)).await?;
    let mut device_list = DeviceList::default();

    if devices.is_empty() {
        return Ok(None);
    }

    for device in devices {
        // Filter meshcore devices starting with "MeshCore" if filter is true
        if filter {
            if device.starts_with("MeshCore") {
                let mut d = Device::default();
                d.name = device;
                device_list.devices.push(d);
            }
        } else {
            let mut d = Device::default();
            d.name = device;
            device_list.devices.push(d);
        }
    }

    Ok(Some(device_list))
}

/// Connect to a meshcore device
pub async fn connect_mc_device(device: &Device) -> Result<MeshCore, meshcore_rs::error::Error> {
    let mc = MeshCore::ble_connect(&device.name).await?;
    // let mut device = device.clone();
    // device.is_connected = true;
    Ok(mc)
}

/// Deisconnect from meschcore device
pub async fn disconnect_mc_device(mc: MeshCore) -> Result<(), meshcore_rs::error::Error> {
    let _ = mc.disconnect().await?;
    Ok(())
}
