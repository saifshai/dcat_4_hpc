use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewLineageDependency {
    pub downstream_version_id: i64,
    pub upstream_product_uri: String,
    pub upstream_version: Option<String>,
}

impl NewLineageDependency {
    /// Creates a NewLineageDependency instance for database insertion, excluding fields handled by the database layer.
    pub fn new(
        downstream_version_id: i64,
        upstream_product_uri: String,
        upstream_version: Option<String>,
    ) -> Self {
        Self {
            downstream_version_id,
            upstream_product_uri,
            upstream_version,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::NewLineageDependency;

    #[test]
    // Verifies the constructor leaves optional fields unset and preserves input values.
    fn new_lineage_dependency_initializes_insert_model_with_optional_fields_to_none() {
        let dependency = NewLineageDependency::new(4, "mesh://orders".to_string(), None);

        assert_eq!(dependency.downstream_version_id, 4);
        assert_eq!(dependency.upstream_product_uri, "mesh://orders");
        assert_eq!(dependency.upstream_version, None);
    }

    #[test]
    // Verifies the constructor preserves optional user-provided values.
    fn new_lineage_dependency_preserves_optional_fields() {
        let dependency =
            NewLineageDependency::new(4, "mesh://orders".to_string(), Some("v2".to_string()));

        assert_eq!(dependency.upstream_version, Some("v2".to_string()));
    }
}
