use crate::structure::{Package, PckpDbClient};

#[get("/api/package/<package_name>")]
pub async fn get_package(conn: PckpDbClient, package_name: String) -> Result<String, String> {
    conn.run(move |c| {
        match &c.query_one(
            r#"SELECT * FROM pckp.package WHERE package_name = $1;"#,
            &[&package_name],
        ) {
            Ok(query) => {
                let package = Package {
                    added_timestamp: query.get::<_, i64>("added_timestamp"),
                    repo_name: query.get::<_, String>("repo_name"),
                    homepage: query.get::<_, String>("homepage"),
                    author_id: query.get::<_, i32>("author_id"),
                    downloads: query.get::<_, i32>("downloads"),
                    name: query.get::<_, String>("name"),
                    id: query.get::<_, i32>("id"),
                };

                println!("{:?}", package);

                return Ok(serde_json::to_string(&package).unwrap());
            }
            Err(_) => {
                return Err(r#"{"error": {"message": "This package does not exist."}}"#.to_owned())
            }
        };
    })
    .await
}

// #[get("/api/package/<package>/download")]
// pub async fn get_package_dl(conn: PckpDbClient, package: String) -> Result<String, String> {
//     conn.run(move |c| {
//         let (name, version) = parse_package_name(package);

//         if let Ok(res) = c.execute(
//             "SELECT * FROM pckp.package package_name = $1",
//             &[&name],
//         ) {
//             c.execute(
//                 "UPDATE pckp.package SET package_downloads = package_downloads + 1 WHERE package_name = $1",
//                 &[&name],
//             )
//             .unwrap();

//             return Ok("".to_string())
//         }

//         Err(r#"{"error": {"message": "This package does not exist."}}"#.to_owned())
//     }).await
// }

#[get("/api/new/package")]
pub fn get_new_package() -> &'static str {
    "UwU"
}

#[post("/api/new/package")]
pub async fn post_new_package(_conn: PckpDbClient) -> String {
    "uwuwuwu".to_owned()
}
