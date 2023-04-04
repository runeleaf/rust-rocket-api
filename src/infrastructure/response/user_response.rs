use rocket::serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub id: String,
    pub name: String,
    pub age: i32,
}
