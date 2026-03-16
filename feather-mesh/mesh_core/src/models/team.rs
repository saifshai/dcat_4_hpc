use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub team_id: Option<i32>,
    pub name: String,
    pub description: String,
    pub created_at: Option<NaiveDateTime>,
}

impl Team {
    pub fn new(name: String, description: String) -> Self {
        Self {
            team_id: None, // to be set by the database layer
            name,
            description,
            created_at: None, // to be set by the database layer
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Team;
    use chrono::NaiveDateTime;

    #[test]
    // Verifies the constructor leaves DB-managed fields unset and preserves input values.
    fn new_initializes_database_managed_fields_to_none() {
        let team = Team::new("Test Team".to_string(), "Test description".to_string());

        assert_eq!(team.team_id, None);
        assert_eq!(team.name, "Test Team");
        assert_eq!(team.description, "Test description");
        assert_eq!(team.created_at, None);
    }

    #[test]
    // Confirms serde preserves the model shape and timestamp values through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let created_at =
            NaiveDateTime::parse_from_str("2026-03-11 10:15:30", "%Y-%m-%d %H:%M:%S").unwrap();
        let team = Team {
            team_id: Some(7),
            name: "Test Team".to_string(),
            description: "Test description".to_string(),
            created_at: Some(created_at),
        };

        let json = serde_json::to_string(&team).unwrap();
        let round_trip: Team = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"team_id\":7"));
        assert!(json.contains("\"name\":\"Test Team\""));
        assert_eq!(round_trip.team_id, Some(7));
        assert_eq!(round_trip.name, "Test Team");
        assert_eq!(round_trip.description, "Test description");
        assert_eq!(round_trip.created_at, Some(created_at));
    }
}
