use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewMetadata {
    pub data_product_version_id: i64,
    pub namespace: Option<String>,
    pub meta_key: String,
    pub meta_value: Option<String>,
    pub value_type: Option<String>,
}

impl NewMetadata {
    /// Creates a NewMetadata instance for database insertion, excluding fields handled by the database layer.
    pub fn new(
        data_product_version_id: i64,
        namespace: Option<String>,
        meta_key: String,
        meta_value: Option<String>,
        value_type: Option<String>,
    ) -> Self {
        Self {
            data_product_version_id,
            namespace,
            meta_key,
            meta_value,
            value_type,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewMetadata;

    #[test]
    // Verifies the constructor leaves optional fields unset and preserves input values.
    fn new_metadata_initializes_insert_model_with_optional_fields_to_none() {
        let metadata = NewMetadata::new(42, None, "owner".to_string(), None, None);

        assert_eq!(metadata.data_product_version_id, 42);
        assert_eq!(metadata.namespace, None);
        assert_eq!(metadata.meta_key, "owner");
        assert_eq!(metadata.meta_value, None);
        assert_eq!(metadata.value_type, None);
    }

    #[test]
    // Verifies the constructor preserves optional user-provided values.
    fn new_metadata_preserves_optional_fields() {
        let metadata = NewMetadata::new(
            42,
            Some("dq".to_string()),
            "threshold".to_string(),
            Some("30".to_string()),
            Some("integer".to_string()),
        );

        assert_eq!(metadata.namespace, Some("dq".to_string()));
        assert_eq!(metadata.meta_value, Some("30".to_string()));
        assert_eq!(metadata.value_type, Some("integer".to_string()));
    }
}
