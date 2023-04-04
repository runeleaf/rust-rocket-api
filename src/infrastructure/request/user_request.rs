use rocket::serde::Deserialize;

#[derive(Deserialize)]
pub struct UserRequest {
    pub name: String,
    pub age: i32,
}
