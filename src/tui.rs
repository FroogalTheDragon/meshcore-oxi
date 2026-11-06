use crate::ble::DeviceList;
use crate::contacts::ContactsList;
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
use meshcore_rs::PayloadType::TextMsg;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{
    Block, Borders, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph,
    StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};

#[derive(Clone, Debug, Default)]
pub struct BleConnectionScreen {
    device_list: DeviceList,
}

#[derive(Debug, Default, Clone)]
pub struct ContactsScreen {
    contacts_list: ContactsList,
}

#[derive(Debug)]
pub enum Screens {
    BleConnectionScreen(BleConnectionScreen),
    ContactsScreen(ContactsScreen),
}

impl Widget for BleConnectionScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::raw("Bluetooth Screen").render(area, buf);
    }
}

impl Widget for ContactsScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::raw("Contacts List").render(area, buf);
    }
}

impl Widget for &Screens {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            Screens::BleConnectionScreen(blecs) => {
                blecs.clone().render(area, buf);
            }
            Screens::ContactsScreen(cs) => {
                cs.clone().render(area, buf);
            }
        }
    }
}

impl Default for Screens {
    fn default() -> Self {
        // Screens::BleConnectionScreen(BleConnectionScreen { available_devices: DeviceList::default() })
        Screens::ContactsScreen(ContactsScreen {
            contacts_list: ContactsList::default(),
        })
    }
}

impl BleConnectionScreen {
    pub fn new(device_list: DeviceList) -> Self {
        Self { device_list }
    }
}
