#[macro_use]
extern crate rocket;
use rocket::build;

mod routes;
use routes::*;

mod structure;
use structure::*;

#[launch]
async fn rocket() -> _ {
    build()
        .attach(PckpDbClient::fairing())
        .mount("/", routes![base::route, api::get_new_package, api::get_package_meta, api::get_package_dl, api::post_new_package])
}
