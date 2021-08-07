use std::collections::HashMap;

use postgres::Client;
use rocket_sync_db_pools::database;
use serde::{Serialize, Deserialize};

#[database("krista")]
pub struct PckpDbClient(Client);

#[derive(Debug, FromForm, Deserialize, Serialize)]
pub struct PackageMetaRow {
    pub package_id: i32,
    pub package_name: String,
    pub package_desc: String,
    pub homepage: String,
    pub added_at: i64,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
pub struct PackageMetadata {
    pub package_name: String,
    pub package_desc: String,
    pub homepage: String,
    pub added_at: i64,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
pub struct UserRow {
    pub user_id: i32,
    pub user_name: String,
    pub user_profile_img: String,
    pub verified: bool,
    pub github_url: String,
}

#[derive(Debug, FromForm, Deserialize, Serialize)]
pub struct NewPackage {
    pub meta: PackageMetadata,
    pub owner: UserRow,
    pub file_tree: HashMap<String, String>
}
