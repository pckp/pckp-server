#![feature(async_closure)]

use std::{fs::File, io::Read, path::PathBuf};

use crate::structure::{NewPackage, PackageMetaRow, PckpDbClient};
use rocket::{Response, fs::NamedFile, serde::json::Json};
use tar::Builder;

#[get("/api/package/<package>")]
pub async fn get_package_meta(conn: PckpDbClient, package: String) -> String {
    conn.run(move |c| {
        match &c.query_one(
            r#"SELECT * FROM pckp.package WHERE package_name = $1;"#,
            &[&package],
        ) {
            Ok(query) => {
                let package_meta = PackageMetaRow {
                    added_at: query.get::<_, i64>("added_at"),
                    homepage: query.get::<_, &str>("homepage").to_owned(),
                    package_desc: query.get::<_, &str>("package_desc").to_owned(),
                    package_name: query.get::<_, &str>("package_name").to_owned(),
                    package_id: query.get::<_, i32>("package_id").to_owned(),
                };

                println!("{:?}", package_meta);

                return serde_json::to_string(&package_meta).unwrap();
            }
            Err(e) => {
                eprintln!("{}", e);
            }
        };

        r#"{"error": {"message": "This package does not exist."}}"#.to_owned()
    })
    .await
}

#[get("/api/package/<package>/download")]
pub async fn get_package_dl(conn: PckpDbClient, package: String) -> Result<Vec<u8>, String> {
    conn.run(move |c| {
        let (name, version) = parse_package_name(package);

        let path_pkg_name = PathBuf::from("packages").join(&name);
        let mut path_pkg_ver = path_pkg_name.join(&version);

        path_pkg_ver.set_extension("tar");

        if path_pkg_name.exists() && path_pkg_ver.exists() {
            c.execute(
                "UPDATE pckp.package SET package_downloads = package_downloads + 1 WHERE package_name = $1",
                &[&name],
            )
            .unwrap();

            let mut buf = Vec::new();
            let mut file = File::open(path_pkg_ver).unwrap();
            file.read_to_end(&mut buf).unwrap();
            
            return Ok(buf);
        }

        Err(r#"{"error": {"message": "This package does not exist."}}"#.to_owned())
    }).await
}

fn parse_package_name(package: String) -> (String, String) {
    let mut split = package.split("@");
    let first = split.next();
    let second = split.next();

    (
        first.unwrap().to_string(),
        if second.is_some() {
            second.unwrap().to_string()
        } else {
            String::from("latest")
        },
    )
}

#[get("/api/new/package")]
pub fn get_new_package() -> &'static str {
    "UwU"
}

#[post("/api/new/package", data = "<form>")]
pub async fn post_new_package(_conn: PckpDbClient, form: Json<NewPackage>) -> String {

    "uwuwuwu".to_owned()
}
