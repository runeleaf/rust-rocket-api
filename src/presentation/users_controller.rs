use rocket::serde::json::Json;
use rocket::*; // ::{get, post, launch, catch, routes};

use sqlx::{Pool, Postgres};

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
    let users = sqlx::query_as!(User, "SELECT id, name, age FROM users order by id desc")
        .fetch_all(&**pool)
        .await;

    println!("{:?}", users);
    (http::Status::Ok, Json(users.unwrap()))
}

#[post("/users", data = "<user>")]
pub async fn create_user(
    pool: &State<Pool<Postgres>>,
    user: Json<UserRequest>,
) -> (http::Status, Json<UserResponse>) {
    let new_user = sqlx::query_as!(
        User,
        "INSERT INTO users (name, age, created_at, updated_at) VALUES ($1, $2, now(), now()) RETURNING id, name, age",
        user.name,
        user.age
    )
    .fetch_one(&**pool)
    .await;

    let new_user = new_user.unwrap();

    (
        http::Status::Created,
        Json(UserResponse {
            id: new_user.id.to_string(),
            name: new_user.name,
            age: new_user.age,
        }),
    )
}
