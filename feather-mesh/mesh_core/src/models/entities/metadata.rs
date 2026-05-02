use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Metadata {
    pub metadata_id: i64,
    pub data_product_version_id: i64,
    pub namespace: Option<String>,
    pub meta_key: String,
    pub meta_value: Option<String>,
    pub value_type: Option<String>,
    pub created_at: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::Metadata;
    use chrono::NaiveDateTime;

    #[test]
    // Confirms serde preserves the model shape and timestamp values through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let created_at =
            NaiveDateTime::parse_from_str("2026-03-11 09:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let metadata = Metadata {
            metadata_id: 3,
            data_product_version_id: 9,
            namespace: Some("dq".to_string()),
            meta_key: "threshold".to_string(),
            meta_value: Some("30".to_string()),
            value_type: Some("integer".to_string()),
            created_at,
        };

        let json = serde_json::to_string(&metadata).unwrap();
        let round_trip: Metadata = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"metadata_id\":3"));
        assert!(json.contains("\"data_product_version_id\":9"));
        assert!(json.contains("\"meta_key\":\"threshold\""));
        assert_eq!(round_trip, metadata);
    }
}
