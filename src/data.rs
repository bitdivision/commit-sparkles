/// Module containing all API data structures.
use github::GithubCredentials;

use postgres::Connection;

#[derive(Debug, Clone, Deserialize)]
pub struct GetToken {
    pub code: String,
    pub state: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub github: GithubCredentials,
}

impl User {

    pub fn insert(&self, connection: &Connection) {
        let query = include_str!("../sql/create_user.sql");

        connection.execute(query, &[
                           &self.github.access_token,
                           &self.github.scope,
                           &self.github.token_type])
            .expect("Could not insert user into database");
    }


    pub fn create(connection: &Connection, credentials: GithubCredentials) -> User {

        let user = User {
            github: credentials
        };
        user.insert(connection);
        user
    }

}

