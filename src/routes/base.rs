#[get("/")]
pub async fn route() -> &'static str {
    "Hello, pckp!"
}
