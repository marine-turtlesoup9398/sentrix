use rusqlite::{params, Connection};
use sentrix_core::Result;
use sentrix_ir::FileItem;
use std::path::Path;

pub struct SqliteStore {
    conn: Connection,
}

impl SqliteStore {
    pub fn open<P: AsRef<Path>>(db_path: P) -> Result<Self> {
        let conn = Connection::open(db_path)
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        let store = Self { conn };
        store.init_tables()?;
        Ok(store)
    }

    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        let store = Self { conn };
        store.init_tables()?;
        Ok(store)
    }

    fn init_tables(&self) -> Result<()> {
        self.conn
            .execute_batch(
                "
            CREATE TABLE IF NOT EXISTS file_cache (
                relative_path TEXT PRIMARY KEY,
                hash TEXT NOT NULL,
                size_bytes INTEGER NOT NULL,
                line_count INTEGER NOT NULL,
                sir_json TEXT NOT NULL,
                updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
            );

            CREATE TABLE IF NOT EXISTS graph_nodes (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                node_type TEXT NOT NULL,
                path TEXT
            );

            CREATE TABLE IF NOT EXISTS graph_edges (
                source_id TEXT NOT NULL,
                target_id TEXT NOT NULL,
                edge_type TEXT NOT NULL,
                PRIMARY KEY (source_id, target_id, edge_type)
            );
            ",
            )
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        Ok(())
    }

    pub fn get_cached_hash(&self, relative_path: &str) -> Option<String> {
        let mut stmt = self
            .conn
            .prepare("SELECT hash FROM file_cache WHERE relative_path = ?1")
            .ok()?;
        stmt.query_row(params![relative_path], |row| row.get(0))
            .ok()
    }

    pub fn get_cached_file(&self, relative_path: &str) -> Option<FileItem> {
        let mut stmt = self
            .conn
            .prepare("SELECT sir_json FROM file_cache WHERE relative_path = ?1")
            .ok()?;
        let json_str: String = stmt
            .query_row(params![relative_path], |row| row.get(0))
            .ok()?;
        serde_json::from_str(&json_str).ok()
    }

    pub fn save_file(&self, file: &FileItem) -> Result<()> {
        let json_str = serde_json::to_string(file)
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        self.conn.execute(
            "INSERT INTO file_cache (relative_path, hash, size_bytes, line_count, sir_json)
             VALUES (?1, ?2, ?3, ?4, ?5)
             ON CONFLICT(relative_path) DO UPDATE SET
             hash = ?2, size_bytes = ?3, line_count = ?4, sir_json = ?5, updated_at = CURRENT_TIMESTAMP",
            params![
                file.relative_path,
                file.hash,
                file.size_bytes as i64,
                file.line_count as i64,
                json_str
            ],
        ).map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        Ok(())
    }

    pub fn get_all_cached_files(&self) -> Result<Vec<FileItem>> {
        let mut stmt = self
            .conn
            .prepare("SELECT sir_json FROM file_cache")
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        let rows = stmt
            .query_map([], |row| {
                let json_str: String = row.get(0)?;
                Ok(json_str)
            })
            .map_err(|e| sentrix_core::SentrixError::Storage(e.to_string()))?;

        let mut files = Vec::new();
        for json_str in rows.flatten() {
            if let Ok(f) = serde_json::from_str::<FileItem>(&json_str) {
                files.push(f);
            }
        }
        Ok(files)
    }
}
