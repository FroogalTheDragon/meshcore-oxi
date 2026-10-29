use futures::io;
use ratatui::DefaultTerminal;

use crate::tui::{BleConnectionScreen, ContactsScreen, Screens};

pub struct App {
    user_exited: bool,
    screens: Screens,
}

impl App {
    pub fn new(user_exited: bool) -> Self {
        Self {
            user_exited,
            screens: Screens::default()
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        todo!("Implement handle_events function")
    }
}
