use rocket::serde::json::Json;
use rocket::*; // ::{get, post, launch, catch, routes};

use sqlx::{Pool, Postgres};
use rocket_session_store::{Session, SessionStore};

use crate::infrastructure::request::user_request::UserRequest;
use crate::infrastructure::response::user_response::UserResponse;

use crate::domain::model::user::User;

#[get("/users/<id>")]
pub async fn show_user(id: i32, pool: &State<Pool<Postgres>>) -> (http::Status, Json<User>) {
    let user = sqlx::query_as!(User, "SELECT id, name, age FROM users where id = $1", id)
        .fetch_one(&**pool)
        .await;
    (http::Status::Ok, Json(user.unwrap()))
}

#[get("/users")]
pub async fn index_user(pool: &State<Pool<Postgres>>) -> (http::Status, Json<Vec<User>>) {
    let name = session.get();
    println!("{:?}", pool);
    let users = sqlx::query_as!(User, "SELECT id, name, age FROM users order by id desc")
        .fetch_all(&**pool)
        .await;

    println!("{:?}", users);
    (http::Status::Ok, Json(users.unwrap()))
}

#[post("/users", data = "<user>")]
pub fn create_user(user: Json<UserRequest>) -> (http::Status, Json<UserResponse>) {
    (
        http::Status::Created,
        Json(UserResponse {
            id: "1".to_string(),
            name: user.name.clone(),
            age: user.age.clone(),
        }),
    )
}
