use rocket::serde::json::Json;
use rocket::*; // ::{get, post, launch, catch, routes};

use crate::infrastructure::response::message_response::MessageResponse;

#[get("/")]
pub fn root() -> (http::Status, Json<MessageResponse>) {
    (
        http::Status::Ok,
        Json(MessageResponse {
            message: "Hello, world!".to_string(),
        }),
    )
}

#[get("/_healthcheck")]
pub fn healthcheck() -> &'static str {
    "OK"
}
