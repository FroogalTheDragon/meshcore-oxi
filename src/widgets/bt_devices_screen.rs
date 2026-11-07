use ratatui::{buffer::Buffer, layout::Rect, widgets::{Block, Widget}};
use crate::{DeviceList, tui::Screens};

#[derive(Clone, Debug, Default)]
pub struct BleConnectionScreen {
    pub device_list: DeviceList,
}

impl Widget for BleConnectionScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Screens::clear(area, buf);
        Block::bordered().title("Devices").render(area, buf);
    }
}

impl BleConnectionScreen {
    pub fn new(device_list: DeviceList) -> Self {
        Self { device_list }
    }

    /// Build a device list and set the result to BleConnectionScreen::device_list
    pub async fn build_device_list(&mut self) -> Result<(), meshcore_rs::Error> {
        match DeviceList::find_mc_devices(5, false).await? {
            Some(devices) => {
                self.device_list = devices;
                Ok(())
            }
            None => {
                self.device_list = DeviceList::default();
                Ok(())
            }
        }
    }
}
