use postgres::Client;
use rocket_sync_db_pools::database;
use serde::{Serialize, Deserialize};

#[database("krista")]
pub struct PckpDbClient(Client);

#[derive(Debug, Serialize, Deserialize)]
pub struct Package {
    pub id: i32,
    pub author_id: i32,
    pub name: String,
    pub repo_name: String,
    pub homepage: String,
    pub added_timestamp: i64,
    pub downloads: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Author {
    id: i32,
    name: String,
    github_user: String,
    verified: bool,
    profile_image: String,
}
