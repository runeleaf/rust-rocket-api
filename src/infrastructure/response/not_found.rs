use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct NotFound {
    pub message: String,
    pub debug: String,
}
