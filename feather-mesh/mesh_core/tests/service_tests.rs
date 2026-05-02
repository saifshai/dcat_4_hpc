use std::fs;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use mesh_core::init_db;
use mesh_core::services::RegistryService;
use rusqlite::Connection;

// Builds an isolated temporary database path for each test.
fn unique_test_db_path() -> PathBuf {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("System time before UNIX EPOCH")
        .as_nanos();
    let mut path = std::env::temp_dir();
    path.push(format!("mesh_core_service_{}.db", timestamp));
    path
}

// Opens a fresh test database and applies the registry schema.
fn test_connection() -> (Connection, PathBuf) {
    let path = unique_test_db_path();
    let conn = init_db(&path).expect("Failed to initialize service test database");
    (conn, path)
}

#[test]
// Verifies the service exposes a CLI-friendly workflow for registering teams.
fn registry_service_registers_and_lists_teams() {
    let (conn, path) = test_connection();
    let service = RegistryService::new(&conn); // init service instance with test db connection

    let team = service
        .register_team("Climate".to_string())
        .expect("Failed to register team");

    assert!(team.team_id > 0);
    assert_eq!(team.name, "Climate");

    let teams = service.list_teams().expect("Failed to list teams");
    assert_eq!(teams, vec![team]);

    fs::remove_file(path).ok();
}
