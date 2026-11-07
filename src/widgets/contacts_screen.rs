use crate::contacts::ContactsList;
use ratatui::{buffer::Buffer, layout::Rect, text::Text, widgets::Widget};

#[derive(Debug, Default, Clone)]
pub struct ContactsScreen {
    pub contacts_list: ContactsList,
}

impl Widget for ContactsScreen {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Text::raw("Contacts List").render(area, buf);
    }
}
