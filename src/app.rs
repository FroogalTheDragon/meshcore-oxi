use crate::{
    ble::Device,
    theme::THEME,
    tui::Screens,
    widgets::{bt_devices_screen::BleConnectionScreen, contacts_screen::ContactsScreen},
};
use color_eyre::eyre::{Context, Result};
use futures::io;
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::Style,
    text::Text,
    widgets::{Block, Clear, Widget},
};
use ratatui_core::text::{Line, Span};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

// #[derive(Default)]
pub struct App {
    connected_device: Option<Device>,
    user_exited: bool,
    screen: Screens,
    ble_connection_screen: BleConnectionScreen,
    contacts_screen: ContactsScreen,
    mode: Mode,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

// Default app state
impl Default for App {
    /// Define the default state of the app
    fn default() -> Self {
        Self {
            connected_device: None,
            user_exited: false,
            screen: Screens::default(),
            ble_connection_screen: BleConnectionScreen::default(),
            contacts_screen: ContactsScreen::default(),
            mode: Mode::default(),
        }
    }
}

/// Run/Event logic
impl App {
    pub fn new(
        connected_device: Option<Device>,
        user_exited: bool,
        screen: Screens,
        ble_connection_screen: BleConnectionScreen,
        contacts_screen: ContactsScreen,
        mode: Mode,
    ) -> Self {
        Self {
            connected_device,
            user_exited,
            screen,
            ble_connection_screen,
            contacts_screen,
            mode,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            if self.user_exited {
                break;
            } else {
                terminal
                    .draw(|frame| self.render(frame))
                    .wrap_err("terminal.draw")?;
                self.handle_events()?;
            };
        }
        Ok(())
    }

    fn is_running(&self) -> bool {
        self.mode != Mode::Quit
    }

    fn handle_events(&mut self) -> io::Result<()> {
        // todo!("Implement handle_events function")
        Ok(())
    }

    fn render(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(0),
            Constraint::Length(1),
        ]);

        let [title_bar, tab, bottom_bar] = area.layout(&layout);

        Block::new().style(THEME.root).render(area, buf);
        self.render_title_bar(title_bar, buf);
        self.render_selected_tab(tab, buf);
        App::render_bottom_bar(bottom_bar, buf);
    }
}

/// Render Logic
impl App {
    /// Render App Title bar
    fn render_title_bar(&self, area: Rect, buf: &mut Buffer) {
        let line = Line::from("Crabcore").style(Style::new().yellow());
        line.render(area, buf);
    }

    /// Render the selected tab, the main content
    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        match &self.screen {
            Screens::BleConnectionScreen(blect) => blect.clone().render(area, buf),
            Screens::ContactsScreen(cts) => cts.clone().render(area, buf),
        }
    }

    /// Render bottom app bar
    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {
        Text::raw("Bottom Control Bar").render(area, buf);
    }
}
