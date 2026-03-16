use std::fs;
use std::path::PathBuf;

#[test]
fn test_can_read_static_fixture_data() {
    // Get the path to the mesh_core directory at compile time
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests/data/lab_a/canada_climate_observations_daily.csv");

    // Read the file content
    let content = fs::read_to_string(&path)
        .expect("Failed to read the fixture file. Check if the path is correct.");

    // Validate the file contains the expected deterministic data
    assert!(
        content.contains("date,station,temperature_c,precipitation_mm"),
        "CSV header is missing or incorrect"
    );
    assert!(
        content.contains("2026-03-01,OTTAWA-INTL,-5.0,2.5"),
        "Expected data row is missing"
    );
}
