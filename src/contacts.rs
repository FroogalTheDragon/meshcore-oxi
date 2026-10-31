#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Path {
    Direct,
    NHops,
    #[default]
    Flood
}

#[derive(Debug, Clone)]
pub struct Contact {
    // avatar: ?
    name: String,
    is_favorited: bool,
    has_location: bool,
    ke_prefix: String,
    path: Path,
    last_messaged: String
}

#[derive(Debug, Default, Clone)]
pub struct ContactsList {
    contacts: Vec<Contact>,
    count: u32
}