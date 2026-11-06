use crate::ble::Device;
use crate::error::CrabCoreErrors;
use turso::{Builder, Connection};

#[derive(Debug, Default)]
pub struct CrabCoreDb {
    path: String,
}

impl CrabCoreDb {
    pub fn new(path: &str) -> Self {
        Self {
            path: String::from(path),
        }
    }

    pub async fn connect(&self) -> Result<Connection, Box<dyn std::error::Error>> {
        let db = Builder::new_local(&self.path).build().await?;
        let conn = db.connect()?;
        Ok(conn)
    }

    pub async fn create_device_table(db: Connection) -> Result<u64, turso::Error> {
        Ok(db
            .execute(
                "CREATE TABLE IF NOT EXISTS devices (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            is_connected BOOLEAN,
            is_trusted BOOLEAN
        )",
                (),
            )
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ble::Device;
    use std::fs::File;

    /// Create a tests database file to run tests on
    async fn create_test_db_conn(path: &str) -> Connection {
        // Create test database file
        let _ = File::create(path).expect("Failed to created test database");
        let test_db = CrabCoreDb::new(path);
        let test_conn = test_db.connect().await.unwrap();
        test_conn
    }

    /// Cleanup test database files
    fn cleanup_test_db(path: &str) {
        std::fs::remove_file(path).unwrap_or_default();
        std::fs::remove_file(format!("{path}-wal")).unwrap_or_default();
    }

    #[tokio::test]
    async fn test_connect() {
        let test_db_path = "test_connect.db";
        let test_conn = CrabCoreDb::new(test_db_path).connect().await.unwrap();
        assert_eq!(
            test_conn
                .execute("CREATE TABLE IF NOT EXISTS test (test TEXT NOT NULL)", ())
                .await
                .unwrap(),
            0
        );
        // Clean up
        cleanup_test_db(test_db_path);
    }

    #[tokio::test]
    async fn test_create_device_table() {
        let test_db_path = "create_dev_table_test.db";
        let test_conn = create_test_db_conn(test_db_path).await;
        let result = CrabCoreDb::create_device_table(test_conn).await.unwrap();
        assert_eq!(result, 0);
        cleanup_test_db(test_db_path);
    }
}
