enum Path {
    Direct,
    NHops,
    Flood
}

pub struct Contact {
    // avatar: ?
    name: String,
    is_favorited: bool,
    has_location: bool,
    ke_prefix: String,
    path: Path,
    // last_seen: ?
    unread_message_count: u32,
}

pub struct ContactsList {
    contacts: Vec<Contact>,
    count: u32
}