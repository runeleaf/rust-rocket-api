use rocket::serde::json::Json;
use rocket::*; // ::{get, post, launch, catch, routes};

use crate::infrastructure::response::not_found::NotFound;

#[catch(404)]
pub fn response_not_found(req: &Request) -> (http::Status, Json<NotFound>) {
    (
        http::Status::NotFound,
        Json(NotFound {
            message: "Not found".to_string(),
            debug: format!("{}: {}", http::Status::NotFound, req.uri()),
        }),
    )
}

#[catch(default)]
pub fn response_internal_server_error(
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
