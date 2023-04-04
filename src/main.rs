use dotenv::dotenv;
use rocket::serde::json::Json;
use rocket::*; // ::{get, post, launch, catch, routes};
use std::env;

// FIXME:
// use rocket_db_pools::{Connection, Database};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

// use rocket::serde::{Serialize, Deserialize};

mod infrastructure;
use crate::infrastructure::request::user_request::UserRequest;
use crate::infrastructure::response::{
    message_response::MessageResponse, not_found::NotFound, user_response::UserResponse,
};

mod domain;
use crate::domain::model::user::User;

#[get("/")]
fn root() -> (http::Status, Json<MessageResponse>) {
    (
        http::Status::Ok,
        Json(MessageResponse {
            message: "Hello, world!".to_string(),
        }),
    )
}

#[get("/_healthcheck")]
fn healthcheck() -> &'static str {
    "OK"
}

#[catch(404)]
fn response_not_found(req: &Request) -> (http::Status, Json<NotFound>) {
    (
        http::Status::NotFound,
        Json(NotFound {
            message: "Not found".to_string(),
            debug: format!("{}: {}", http::Status::NotFound, req.uri()),
        }),
    )
}

#[catch(default)]
fn response_internal_server_error(
    status: http::Status,
    req: &Request,
) -> (http::Status, Json<NotFound>) {
    (
        status,
        Json(NotFound {
            message: "An error occurred".to_string(),
            debug: format!("{}: {}", status, req.uri()),
        }),
    )
}

#[get("/users/<id>")]
async fn show_user(id: i32, pool: &State<Pool<Postgres>>) -> (http::Status, Json<User>) {
    let user = sqlx::query_as!(User, "SELECT id, name, age FROM users where id = $1", id)
        .fetch_one(&**pool)
        .await;
    (http::Status::Ok, Json(user.unwrap()))
}

#[get("/users")]
async fn index_user(pool: &State<Pool<Postgres>>) -> (http::Status, Json<Vec<User>>) {
    println!("{:?}", pool);
    let users = sqlx::query_as!(User, "SELECT id, name, age FROM users order by id desc")
        .fetch_all(&**pool)
        .await;

    println!("{:?}", users);
    (http::Status::Ok, Json(users.unwrap()))
}

#[post("/users", data = "<user>")]
fn create_user(user: Json<UserRequest>) -> (http::Status, Json<UserResponse>) {
    (
        http::Status::Created,
        Json(UserResponse {
            id: "1".to_string(),
            name: user.name.clone(),
            age: user.age.clone(),
        }),
    )
}

async fn create_db_pool(database_url: &str) -> Result<Pool<Postgres>, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

#[launch]
async fn rocket() -> _ {
    // load .env file
    dotenv().ok();

    // from .env file get DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // create database pool
    let pool = create_db_pool(&database_url)
        .await
        .expect("Failed to create pool");

    rocket::build()
        .manage(pool)
        .register(
            "/",
            catchers![response_not_found, response_internal_server_error],
        )
        .mount("/", routes![root, healthcheck])
        .mount("/api", routes![index_user, show_user, create_user])
}
