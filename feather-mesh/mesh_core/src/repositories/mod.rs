pub mod data_product_repository;
pub mod data_product_version_repository;
pub mod lineage_dependency_repository;
pub mod metadata_repository;
pub mod team_repository;

use chrono::NaiveDateTime;
use rusqlite::{Result, types::Type};

pub use data_product_repository::DataProductRepository;
pub use data_product_version_repository::DataProductVersionRepository;
pub use lineage_dependency_repository::LineageDependencyRepository;
pub use metadata_repository::MetadataRepository;
pub use team_repository::TeamRepository;

// Parses a datetime string from the database into a NaiveDateTime, returning a rusqlite::Error if parsing fails.
fn parse_naive_datetime(column_index: usize, value: String) -> Result<NaiveDateTime> {
    NaiveDateTime::parse_from_str(&value, "%Y-%m-%d %H:%M:%S").map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(column_index, Type::Text, Box::new(err))
    })
}
