use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_id: Option<i32>,
    pub data_product_id: i32,
    pub meta_key: String,
    pub meta_value: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

impl Metadata {
    pub fn new(data_product_id: i32, meta_key: String, meta_value: String) -> Self {
        Self {
            metadata_id: None, // to be set by the database layer
            data_product_id,
            meta_key,
            meta_value,
            created_at: None, // to be set by the database layer
            updated_at: None, // to be set by the database layer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Metadata;
    use chrono::NaiveDateTime;

    #[test]
    // Verifies the constructor leaves DB-managed fields unset and preserves input values.
    fn new_initializes_database_managed_fields_to_none() {
        let metadata = Metadata::new(42, "owner".to_string(), "test-team".to_string());

        assert_eq!(metadata.metadata_id, None);
        assert_eq!(metadata.data_product_id, 42);
        assert_eq!(metadata.meta_key, "owner");
        assert_eq!(metadata.meta_value, "test-team");
        assert_eq!(metadata.created_at, None);
        assert_eq!(metadata.updated_at, None);
    }

    #[test]
    // Confirms serde preserves the model shape and timestamp values through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let created_at =
            NaiveDateTime::parse_from_str("2026-03-11 09:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let updated_at =
            NaiveDateTime::parse_from_str("2026-03-11 12:30:45", "%Y-%m-%d %H:%M:%S").unwrap();
        let metadata = Metadata {
            metadata_id: Some(3),
            data_product_id: 9,
            meta_key: "test_key".to_string(),
            meta_value: "30".to_string(),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let round_trip: Metadata = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"metadata_id\":3"));
        assert!(json.contains("\"data_product_id\":9"));
        assert!(json.contains("\"meta_key\":\"test_key\""));
        assert_eq!(round_trip.metadata_id, Some(3));
        assert_eq!(round_trip.data_product_id, 9);
        assert_eq!(round_trip.meta_key, "test_key");
        assert_eq!(round_trip.meta_value, "30");
        assert_eq!(round_trip.created_at, Some(created_at));
        assert_eq!(round_trip.updated_at, Some(updated_at));
    }
}
