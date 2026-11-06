#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Path {
    Direct,
    NHops,
    #[default]
    Flood,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub enum ContactType {
    #[default]
    Chat,
    Repeater,
    Room,
    Sensor,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct Contact {
    // avatar: ?
    name: String,
    is_favorited: bool,
    has_location: bool,
    key_prefix: String,
    path: Path,
    last_messaged: String,
    contact_type: ContactType,
}

#[derive(Debug, Default, Clone)]
pub struct ContactsList {
    contacts: Vec<Contact>,
    count: u32,
}

impl ContactsList {
    pub fn new(contacts: Vec<Contact>, count: u32) -> Self {
        Self { contacts, count }
    }
    /// Remove the contact passed to this function from the contact list if found
    pub fn remove_contact(&self, contact: &Contact) -> Self {
        let mut cl = self.clone();
        for (i, c) in self.contacts.iter().enumerate() {
            if c == contact {
                cl.contacts.remove(i);
            }
        }
        cl
    }

    pub fn add_contact(&self, contact: Contact) -> Result<Self, ()> {
        let mut cl = self.clone();
        for c in self.contacts.clone() {
            // Contact is already in the contact list
            if c == contact {
                return Err(());
            }
        }
        cl.contacts.push(contact);
        Ok(cl)
    }
}

impl Contact {
    pub fn new(
        name: String,
        is_favorited: bool,
        has_location: bool,
        key_prefix: String,
        path: Path,
        last_messaged: String,
        contact_type: ContactType,
    ) -> Self {
        Self {
            name,
            is_favorited,
            has_location,
            key_prefix,
            path,
            last_messaged,
            contact_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_contact() {
        let mut contact_vec = vec![];
        for i in 1..5 {
            let mut contact = Contact::default();
            contact.name = format!("Contact {}", i);

            contact_vec.push(contact);
        }

        let test_contact_list = ContactsList::new(contact_vec.clone(), contact_vec.len() as u32);
        assert_eq!(test_contact_list.contacts.len(), 4);
        let test_contact_list = test_contact_list.add_contact(Contact::default()).unwrap();
        assert_eq!(test_contact_list.contacts.len(), 5);
    }

    #[test]
    fn test_remove_contact() {
        let mut contact_vec = vec![];
        for i in 1..5 {
            let mut contact = Contact::default();
            contact.name = format!("Contact {}", i);

            contact_vec.push(contact);
        }

        let test_contact_list = ContactsList::new(contact_vec.clone(), contact_vec.len() as u32);
        assert_eq!(test_contact_list.contacts.len(), 4);
        let test_contact_list = test_contact_list.remove_contact(&test_contact_list.contacts[0]);
        assert_eq!(test_contact_list.contacts.len(), 3);
    }
}
