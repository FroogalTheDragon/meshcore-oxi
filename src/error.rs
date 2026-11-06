use crate::ble::Device;
use std::fmt;

#[derive(Debug)]
pub enum CrabCoreErrors {
    BluetoothConnectionFailed(Device),
    MessageFailedToSend(String),
    FailedToAddContact(String),
    DatabaseError(String),
}

impl fmt::Display for CrabCoreErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CrabCoreErrors::BluetoothConnectionFailed(device) => {
                write!(f, "Failed to connect to {}", device.name)
            }
            CrabCoreErrors::DatabaseError(msg) => write!(f, "Database Error: {}", msg),
            CrabCoreErrors::FailedToAddContact(msg) => write!(f, "Failed to add contact: {}", msg),
            CrabCoreErrors::MessageFailedToSend(msg) => {
                write!(f, "Failed to send message: {}", msg)
            }
        }
    }
}

impl std::error::Error for CrabCoreErrors {}
