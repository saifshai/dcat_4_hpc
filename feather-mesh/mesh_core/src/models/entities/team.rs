use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Team {
    pub team_id: i64,
    pub name: String,
    pub created_at: NaiveDateTime,
}

#[cfg(test)]
mod tests {
    use super::Team;
    use chrono::NaiveDateTime;

    #[test]
    // Confirms serde preserves the model shape and timestamp values through a JSON round trip.
    fn serializes_and_deserializes_with_expected_shape() {
        let created_at =
            NaiveDateTime::parse_from_str("2026-03-11 10:15:30", "%Y-%m-%d %H:%M:%S").unwrap();
        let team = Team {
            team_id: 7,
            name: "Test Team".to_string(),
            created_at,
        };

        let json = serde_json::to_string(&team).unwrap();
        let round_trip: Team = serde_json::from_str(&json).unwrap();

        assert!(json.contains("\"team_id\":7"));
        assert!(json.contains("\"name\":\"Test Team\""));
        assert_eq!(round_trip, team);
    }
}
