/// Module containing all API data structures.

#[derive(Debug, Clone, Deserialize)]
pub struct GetToken {
    pub code: String,
    pub state: String,
}
