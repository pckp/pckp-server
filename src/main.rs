#[macro_use] extern crate rocket;
use rocket::build;

mod routes;
use routes::*;

#[launch]
fn rocket() -> _ {
    build().mount("/", routes![base::route])
}
