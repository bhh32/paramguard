use chrono::{DateTime, Duration, Utc};
use rusqlite::{params, Connection, Result as SqliteResult};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct ArchivedFile {
    pub id: i64,
    pub name: String,
    pub original_path: String,
    pub format: String,
    pub content_hash: String,
    pub archive_date: DateTime<Utc>,
    pub retention_period: i64, // stored as seconds
    pub reason: String,
    pub metadata: String,
}

pub struct ArchiveDb {
    conn: Connection,
}

impl ArchiveDb {
    pub fn new(db_path: &str) -> SqliteResult<Self> {
        let conn = Connection::open(db_path)?;

        // Create tables if they don't exist.
        conn.execute(
            "CREATE TABLE IF NOT EXISTS archived_files (
                id INTEGER PRIMARY KEY,
                name TEXT NOT NULL,
                original_path TEXT NOT NULL,
                format TEXT NOT NULL,
                content_hash TEXT NOT NULL,
                file_content BLOB NOT NULL,
                archive_date TEXT NOT NULL,
                retention_period INTEGER NOT NULL,
                reason TEXT,
                metadata TEXT,
                UNIQUE(name, archive_date)
            )",
            [],
        )?;

        Ok(Self { conn })
    }

    pub fn archive_file(
        &self,
        name: &str,
        path: &PathBuf,
        content: &[u8],
        format: &str,
        retention_days: i64,
        reason: &str,
        metadata: &str,
    ) -> SqliteResult<i64> {
        // Calculate content hash
        let mut hasher = Sha256::new();
        hasher.update(content);
        let hash = format!("{:x}", hasher.finalize());

        self.conn.execute(
            "INSERT INTO archived_files
            (name, origina_path, format, content_hash, file_content, archive_date, retention_period, reason, metadata)
            VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                name,
                path.to_string_lossy().to_string(),
                format,
                hash,
                content,
                Utc::now().to_rfc3339(),
                retention_days * 86400, // convert days to seconds
                reason,
                metadata,
            ]
        )?;

        Ok(self.conn.last_insert_rowid())
    }

    pub fn restore_file(&self, id: i64) -> SqliteResult<(ArchivedFile, Vec<u8>)> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, original_path, format, content_hash, file_content, archive_date, retention_period, reason, metadata
            FROM archived_files
            WHERE id = ?1"
        )?;

        let mut rows = stmt.query([id])?;

        if let Some(row) = rows.next()? {
            let archived_file = ArchivedFile {
                id: row.get(0)?,
                name: row.get(1)?,
                original_path: row.get(2)?,
                format: row.get(3)?,
                content_hash: row.get(4)?,
                archive_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(6)?)
                    .unwrap()
                    .with_timezone(&Utc),
                retention_period: row.get(7)?,
                reason: row.get(8)?,
                metadata: row.get(9)?,
            };

            let content: Vec<u8> = row.get(5)?;

            Ok((archived_file, content))
        } else {
            Err(rusqlite::Error::QueryReturnedNoRows)
        }
    }

    pub fn list_archives(&self) -> SqliteResult<Vec<ArchivedFile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, origina_path, format, content_hash, archive_date,
             retention_period, reaason, metadata
            FROM archived_files
            ORDER BY archive_date DESC",
        )?;

        let archive_iter = stmt.query_map([], |row| {
            Ok(ArchivedFile {
                id: row.get(0)?,
                name: row.get(1)?,
                original_path: row.get(2)?,
                format: row.get(3)?,
                content_hash: row.get(4)?,
                archive_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                retention_period: row.get(6)?,
                reason: row.get(7)?,
                metadata: row.get(8)?,
            })
        })?;

        archive_iter.collect()
    }

    pub fn can_delete(&self, id: i64) -> SqliteResult<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT archive_date, retention_period
            FROM archived_files
            WHERE id = ?1",
        )?;

        let (archive_date, retention_period): (String, i64) =
            stmt.query_row([id], |row| Ok((row.get(0)?, row.get(1)?)))?;

        let archive_date = DateTime::parse_from_rfc3339(&archive_date)
            .unwrap()
            .with_timezone(&Utc);

        let retention_seconds = Duration::seconds(retention_period);

        Ok(Utc::now() - archive_date >= retention_seconds)
    }

    pub fn delete_archive(&self, id: i64) -> SqliteResult<()> {
        if self.can_delete(id)? {
            self.conn
                .execute("DELETE FROM archived_files WHERE id = ?1", [id])?;

            Ok(())
        } else {
            Err(rusqlite::Error::InvalidParameterName(
                "Retention period has not expired".to_string(),
            ))
        }
    }

    pub fn cleanup_expired(&self) -> SqliteResult<usize> {
        let result = self.conn.execute(
            "DELETE FROM archived_files WHERE strftime('%s', 'now') - strftime('%s', archive_date) > retention_period",
            []
        )?;

        Ok(result)
    }

    pub fn search_archives(&self, query: &str) -> SqliteResult<Vec<ArchivedFile>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, original_path, format, content_hash, archive_date,
            retention_period, reason, metadata
            FROM archived_files
            WHERE name LIKE ?1 OR original_path LIKE ?1 OR reason LIKE ?1
            ORDER BY archive_date DESC",
        )?;

        let search_pattern = format!("%{}%", query);
        let archive_iter = stmt.query_map([search_pattern], |row| {
            Ok(ArchivedFile {
                id: row.get(0)?,
                name: row.get(1)?,
                original_path: row.get(2)?,
                format: row.get(3)?,
                content_hash: row.get(4)?,
                archive_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                retention_period: row.get(6)?,
                reason: row.get(7)?,
                metadata: row.get(8)?,
            })
        })?;

        archive_iter.collect()
    }

    pub fn get_archive_info(&self, id: i64) -> SqliteResult<ArchivedFile> {
        let mut stmt = self.conn.prepare(
            "SELECT id, name, original_path, format, content_hash, archive_date,
            retention_period, reason, metadata
            FROM archived_files
            WHERE id = ?1",
        )?;

        stmt.query_row([id], |row| {
            Ok(ArchivedFile {
                id: row.get(0)?,
                name: row.get(1)?,
                original_path: row.get(2)?,
                format: row.get(3)?,
                content_hash: row.get(4)?,
                archive_date: DateTime::parse_from_rfc3339(&row.get::<_, String>(5)?)
                    .unwrap()
                    .with_timezone(&Utc),
                retention_period: row.get(6)?,
                reason: row.get(7)?,
                metadata: row.get(8)?,
            })
        })
    }

    pub fn update_retention_period(&self, id: i64, new_retention_seconds: i64) -> SqliteResult<()> {
        self.conn.execute(
            "UPDATE archived_files
            SET retention_period = ?1
            WHERE id = ?2",
            params![new_retention_seconds, id],
        )?;

        Ok(())
    }

    pub fn get_statistics(&self) -> SqliteResult<ArchiveStatistics> {}
}
