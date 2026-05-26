use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LineageDependency {
    pub dependency_id: i64,
    pub downstream_version_id: i64,
    pub upstream_product_uri: String,
    pub upstream_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::LineageDependency;

    #[test]
    // Confirms serde preserves the model shape through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let dependency = LineageDependency {
            dependency_id: 14,
            downstream_version_id: 9,
            upstream_product_uri: "mesh://payments".to_string(),
            upstream_version: Some("v2".to_string()),
        };

        let json = serde_json::to_string(&dependency).unwrap();
        let round_trip: LineageDependency = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"dependency_id\":14"));
        assert!(json.contains("\"downstream_version_id\":9"));
        assert_eq!(round_trip, dependency);
    }
}
