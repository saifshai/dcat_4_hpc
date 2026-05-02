use rusqlite::{Connection, Result, Row, params};

use crate::models::{Metadata, NewMetadata};

use super::parse_naive_datetime;

pub struct MetadataRepository;

impl MetadataRepository {
    /// Inserts new metadata and returns the persisted metadata row with database-managed fields.
    pub fn create(conn: &Connection, input: NewMetadata) -> Result<Metadata> {
        conn.execute(
            "INSERT INTO metadata
                (data_product_version_id, namespace, meta_key, meta_value, value_type)
             VALUES (?1, ?2, ?3, ?4, ?5)",
            params![
                input.data_product_version_id,
                input.namespace,
                input.meta_key,
                input.meta_value,
                input.value_type
            ],
        )?;

        let metadata_id = conn.last_insert_rowid();
        Self::get_by_id(conn, metadata_id)
    }

    /// Gets persisted metadata by its primary key.
    pub fn get_by_id(conn: &Connection, metadata_id: i64) -> Result<Metadata> {
        conn.query_row(
            "SELECT metadata_id, data_product_version_id, namespace, meta_key,
                    meta_value, value_type, created_at
             FROM metadata
             WHERE metadata_id = ?1",
            params![metadata_id],
            Self::from_row,
        )
    }

    /// Gets all persisted metadata ordered by primary key.
    pub fn get_all(conn: &Connection) -> Result<Vec<Metadata>> {
        let mut stmt = conn.prepare(
            "SELECT metadata_id, data_product_version_id, namespace, meta_key,
                    meta_value, value_type, created_at
             FROM metadata
             ORDER BY metadata_id",
        )?;
        let rows = stmt.query_map([], Self::from_row)?;

        rows.collect()
    }

    /// Maps a database row -> Metadata struct
    fn from_row(row: &Row<'_>) -> Result<Metadata> {
        let created_at: String = row.get("created_at")?;

        Ok(Metadata {
            metadata_id: row.get("metadata_id")?,
            data_product_version_id: row.get("data_product_version_id")?,
            namespace: row.get("namespace")?,
            meta_key: row.get("meta_key")?,
            meta_value: row.get("meta_value")?,
            value_type: row.get("value_type")?,
            created_at: parse_naive_datetime(6, created_at)?,
        })
    }
}
