use color_eyre::eyre::{Context, Result};
use futures::io;
use ratatui::{DefaultTerminal, Frame, buffer::Buffer, layout::{Constraint, Layout, Rect}, widgets::{Widget, Block}};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::{theme::THEME, tui::{BleConnectionScreen, ContactsScreen, Screens}};

#[derive(Default)]
pub struct App {
    user_exited: bool,
    screen: Screens,
    ble_connection_screen: BleConnectionScreen,
    contacts_screen: ContactsScreen,
    mode: Mode
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Running,
    Destroy,
    Quit,
}

#[derive(Debug, Clone, Copy, Default, Display, EnumIter, FromRepr, PartialEq, Eq)]
enum Tab {
    #[default]
    Contacts,
    BtleConnection
}

/// Run/Event logic
impl App {
    pub fn new(user_exited: bool, screen: Screens, ble_connection_screen: BleConnectionScreen, contacts_screen: ContactsScreen, mode: Mode) -> Self {
        Self {
            user_exited,
            screen,
            ble_connection_screen,
            contacts_screen,
            mode
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        while ! self.user_exited {
            terminal.draw(|frame| self.render(frame)).wrap_err("terminal.draw")?;
            self.handle_events()?;
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
        if self.mode == Mode::Destroy {
            // destroy::destroy(frame);
        }
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

    }

    /// Render the selected tab, the main content
    fn render_selected_tab(&self, area: Rect, buf: &mut Buffer) {
        match &self.screen {
            Screens::BleConnectionScreen(blect) => blect.clone().render(area, buf),
            Screens::ContactsScreen(cts) => cts.clone().render(area, buf)
        }
    }

    /// Render bottom app bar
    fn render_bottom_bar(area: Rect, buf: &mut Buffer) {

    }
}
