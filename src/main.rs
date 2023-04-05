use dotenv::dotenv;
use rocket::*; // ::{get, post, launch, catch, routes};
use std::env;

mod application;
mod domain;
mod infrastructure;
mod presentation;

use crate::infrastructure::middleware::postgres::create_db_pool;
use crate::infrastructure::middleware::redis::create_redis_pool;

use crate::presentation::application_controller::*;
use crate::presentation::errors_controller::*;
use crate::presentation::users_controller::*;

#[launch]
async fn rocket() -> _ {
    // load .env file
    dotenv().ok();

    // from .env file get DATABASE_URL
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");

    // create database pool
    let pool = create_db_pool(&database_url).await.expect("Failed to create pool");

    // create session pool
    // let session = create_redis_pool(&redis_url).await.expect("Failed to create redis pool");

    rocket::build()
        .manage(pool)
        // .manage(session)
        .register(
            "/",
            catchers![response_not_found, response_internal_server_error],
        )
        .mount("/", routes![root, healthcheck])
        .mount("/api", routes![index_user, show_user, create_user])
}
