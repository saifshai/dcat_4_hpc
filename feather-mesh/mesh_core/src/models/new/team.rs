use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct NewTeam {
    pub name: String,
}

impl NewTeam {
    /// Creates a NewTeam instance for database insertion, excluding fields handled by the database layer.
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[cfg(test)]
mod tests {
    use super::NewTeam;

    #[test]
    // Verifies the constructor preserves input values for a new insert model.
    fn new_team_initializes_insert_model() {
        let team = NewTeam::new("Test Team".to_string());

        assert_eq!(team.name, "Test Team");
    }
}
