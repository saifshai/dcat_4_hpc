use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataProduct {
    pub product_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub data_format: String,
    pub access_uri: String,
    pub status: String,
    pub classification: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub team_id: i32,
}

impl DataProduct {
    pub fn new(
        name: String,
        description: String,
        data_format: String,
        access_uri: String,
        status: String,
        classification: String,
        team_id: i32,
    ) -> Self {
        Self {
            product_id: None, // to be set by the database layer
            name,
            description,
            data_format,
            access_uri,
            status,
            classification,
            created_at: None, // to be set by the database layer
            updated_at: None, // to be set by the database layer
            team_id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::DataProduct;
    use chrono::NaiveDateTime;

    #[test]
    // Verifies the constructor leaves DB-managed fields unset and preserves input values.
    fn new_initializes_database_managed_fields_to_none() {
        let product = DataProduct::new(
            "Test".to_string(),
            "Test description".to_string(),
            "json".to_string(),
            "s3://test".to_string(),
            "active".to_string(),
            "internal".to_string(),
            5,
        );

        assert_eq!(product.product_id, None);
        assert_eq!(product.name, "Test");
        assert_eq!(product.description, "Test description");
        assert_eq!(product.data_format, "json");
        assert_eq!(product.access_uri, "s3://test");
        assert_eq!(product.status, "active");
        assert_eq!(product.classification, "internal");
        assert_eq!(product.created_at, None);
        assert_eq!(product.updated_at, None);
        assert_eq!(product.team_id, 5);
    }

    #[test]
    // Confirms serde preserves the model shape and timestamp values through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let created_at =
            NaiveDateTime::parse_from_str("2026-03-11 08:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let updated_at =
            NaiveDateTime::parse_from_str("2026-03-11 16:45:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let product = DataProduct {
            product_id: Some(11),
            name: "Test".to_string(),
            description: "Test description".to_string(),
            data_format: "parquet".to_string(),
            access_uri: "abfs://test".to_string(),
            status: "published".to_string(),
            classification: "confidential".to_string(),
            created_at: Some(created_at),
            updated_at: Some(updated_at),
            team_id: 2,
        };

        let json = serde_json::to_string(&product).unwrap();
        let round_trip: DataProduct = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"product_id\":11"));
        assert!(json.contains("\"name\":\"Test\""));
        assert!(json.contains("\"team_id\":2"));
        assert_eq!(round_trip.product_id, Some(11));
        assert_eq!(round_trip.name, "Test");
        assert_eq!(round_trip.description, "Test description");
        assert_eq!(round_trip.data_format, "parquet");
        assert_eq!(round_trip.access_uri, "abfs://test");
        assert_eq!(round_trip.status, "published");
        assert_eq!(round_trip.classification, "confidential");
        assert_eq!(round_trip.created_at, Some(created_at));
        assert_eq!(round_trip.updated_at, Some(updated_at));
        assert_eq!(round_trip.team_id, 2);
    }
}
