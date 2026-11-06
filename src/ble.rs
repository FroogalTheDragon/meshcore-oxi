use meshcore_rs::MeshCore;
use ratatui::widgets::ListState;
use std::time::Duration;
use turso::Connection;

#[derive(Debug, Clone)]
pub struct Device {
    id: String,
    pub name: String,
    pub is_connected: bool,
    pub is_trusted: bool,
}

impl Device {
    pub fn new(id: String, name: String, is_connected: bool, is_trusted: bool) -> Self {
        Self {
            id,
            name,
            is_connected,
            is_trusted,
        }
    }

    /// Connect to a meshcore device
    pub async fn connect_mc_device(device: &Device) -> Result<MeshCore, meshcore_rs::error::Error> {
        let mc = MeshCore::ble_connect(&device.name).await?;
        // let mut device = device.clone();
        // device.is_connected = true;
        Ok(mc)
    }

    /// Deisconnect from meschcore device
    pub async fn disconnect_mc_device(mc: MeshCore) -> Result<(), meshcore_rs::error::Error> {
        let _ = mc.disconnect().await?;
        Ok(())
    }

    /// Insert this devices into the database
    pub async fn insert(&self, db: Connection) -> Result<u64, turso::Error> {
        let result = db
            .execute(
                "INSERT INTO devices (
            id,
            name,
            is_connected,
            is_trusted
        ) VALUES (?1, ?2, ?3, ?4)",
                [
                    self.id.as_str(),
                    self.name.as_str(),
                    self.is_connected.to_string().as_str(),
                    self.is_trusted.to_string().as_str(),
                ],
            )
            .await?;
        Ok(result)
    }
}

impl Default for Device {
    fn default() -> Self {
        Self {
            id: String::default(),
            name: String::default(),
            is_connected: false,
            is_trusted: false,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeviceList {
    pub devices: Vec<Device>,
    state: ListState,
}

impl DeviceList {
    /// Create new DeviceList
    pub fn new(devices: Vec<Device>, state: ListState) -> Self {
        Self { devices, state }
    }

    /// Find meshcore devices, set scan length
    pub async fn find_mc_devices(
        scan_duration: u64,
        filter: bool,
    ) -> Result<Option<Self>, meshcore_rs::error::Error> {
        let devices = MeshCore::ble_discover(Duration::from_secs(scan_duration)).await?;
        let mut device_list = DeviceList::default();

        if devices.is_empty() {
            return Ok(None);
        }

        for device in devices {
            // Filter meshcore devices starting with "MeshCore" if filter is true
            if filter {
                if device.starts_with("MeshCore") {
                    let mut d = Device::default();
                    d.name = device;
                    device_list.devices.push(d);
                }
            } else {
                let mut d = Device::default();
                d.name = device;
                device_list.devices.push(d);
            }
        }

        Ok(Some(device_list))
    }
}

impl Default for DeviceList {
    fn default() -> Self {
        Self {
            devices: Vec::default(),
            state: ListState::default(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::db::CrabCoreDb;
    use std::fs::File;
    use super::*;

    /// Create a tests database file to run tests on
    async fn create_test_db_conn(path: &str) -> Connection {
        // Create test database file
        let _ = File::create(path).expect("Failed to created test database");
        let test_db = CrabCoreDb::new(path);
        let test_conn = test_db.connect().await.unwrap();
        test_conn
    }

    async fn create_test_device_table(conn: &Connection) {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            is_connected BOOLEAN,
            is_trusted BOOLEAN
        )",
            (),
        )
        .await
        .unwrap();
    }

    /// Cleanup test database files
    fn cleanup_test_db(path: &str) {
        std::fs::remove_file(path).unwrap_or_default();
        std::fs::remove_file(format!("{path}-wal")).unwrap_or_default();
    }

    #[tokio::test]
    async fn test_connect_mc_device() {
        todo!("Write test for connect_mc_device()");
    }

    #[tokio::test]
    async fn test_insert_connected_device() {
        let test_db_path = "create_insert_connected_device.db";
        let test_conn = create_test_db_conn(test_db_path).await;
        create_test_device_table(&test_conn).await;
        let test_device = Device::new(String::from("1"), String::from("test-device"), true, false);
        let rows = test_device.insert(test_conn).await.unwrap();
        assert_eq!(rows, 1);
        cleanup_test_db(test_db_path);
    }
}
