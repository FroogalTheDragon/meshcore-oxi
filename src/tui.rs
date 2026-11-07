use crate::ble::{Device, DeviceList};
use crate::widgets::{bt_devices_screen::{BleConnectionScreen}, contacts_screen::ContactsScreen};
use crate::contacts::ContactsList;
use crate::error::CrabCoreErrors;
use color_eyre::Result;
use crossterm::event::{self, KeyCode, KeyEvent};
// use crossterm::terminal::Clear;
use meshcore_rs::PayloadType::TextMsg;
use ratatui::buffer::Buffer;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::palette::tailwind::{BLUE, GREEN, SLATE};
use ratatui::style::{Color, Modifier, Style, Stylize};
use ratatui::text::{Line, Text};
use ratatui::widgets::{
    Block, Borders, Clear, HighlightSpacing, List, ListItem, ListState, Padding, Paragraph, StatefulWidget, Widget, Wrap,
};
use ratatui::{DefaultTerminal, symbols};





#[derive(Debug)]
pub enum Screens {
    BleConnectionScreen(BleConnectionScreen),
    ContactsScreen(ContactsScreen),
}

impl Screens {
    /// Clear the current buffer
    pub fn clear(area: Rect, buf: &mut Buffer) {
        Clear::default().render(area, buf);
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
        Screens::BleConnectionScreen(BleConnectionScreen { device_list: DeviceList::default() })
        // Screens::ContactsScreen(ContactsScreen {
        //     contacts_list: ContactsList::default(),
        // })
    }
}

