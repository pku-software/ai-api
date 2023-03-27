use std::collections::HashMap;

use crate::db;
use crate::CONFIG;
use serde_json::json;
use warp::http::Response;

pub(super) async fn check_token(token: &String) -> Result<(), String> {
    let collection = db::generate_connection(&CONFIG).await;
    let token = token.trim().split(" ").collect::<Vec<&str>>()[1].to_owned();
    let student = db::get_student_by_token(&collection, &token).await;
    if let Some(student) = student {
        db::add_one(&collection, student).await;
        return Ok(());
    } else {
        return Err(format!("Invalid token {:}", token));
    }
}

pub(super) fn token_error() -> Response<String> {
    Response::builder()
        .header("Content-Type", "application/json")
        .status(401)
        .body(
            json!(
                {
                    "status": "failed",
                    "text": "Invalid token"
                }
            )
            .to_string(),
        )
        .unwrap()
}

pub(super) fn bad_request() -> Response<String> {
    Response::builder()
        .header("Content-Type", "application/json")
        .status(400)
        .body(
            json!(
                {
                    "status": "failed",
                    "text": "BAD REQUEST"
                }
            )
            .to_string(),
        )
        .unwrap()
}

pub(super) fn get_string_from_map<'a>(
    map: &'a HashMap<String, String>,
    key: &str,
) -> Result<&'a str, Response<String>> {
    match map.get(key) {
        Some(value) => Ok(value),
        None => Err(bad_request()),
    }
}

pub(super) fn get_i32_from_map(
    map: &HashMap<String, String>,
    key: &str,
) -> Result<i32, Response<String>> {
    let value = map.get(key);
    match value {
        Some(value) => match value.parse::<i32>() {
            Ok(value) => Ok(value),
            Err(_) => Err(bad_request()),
        },
        None => Err(bad_request()),
    }
}
