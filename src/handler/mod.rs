use crate::db;
use crate::db::student::Student;
use crate::db::*;
use crate::CONFIG;
use std::collections::HashMap;
use warp::http::Response;

pub(crate) async fn get_token(map: HashMap<String, String>) -> Response<String> {
    let collection = db::generate_connection(&CONFIG).await;
    let id = match map.get("id") {
        Some(id) => id,
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .header("Status-Code", "400")
                .body("BAD REQUEST".to_owned())
                .unwrap();
        }
    };

    let secret = match map.get("password") {
        Some(secret) => secret,
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .header("Status-Code", "400")
                .body("BAD REQUEST".to_owned())
                .unwrap();
        }
    };

    let secret = secret.clone().trim().to_string();

    if secret != "42" {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "403")
            .body("INVALID PASSWORD".to_owned())
            .unwrap();
    }

    let student = get_student(&collection, id).await;
    if student.is_none() {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "404")
            .body("STUDENT NOT FOUND".to_owned())
            .unwrap();
    }
    let mut student = student.unwrap();
    student.num += 1;
    update_student(&collection, &student).await;
    Response::builder()
        .header("Content-Type", "text/plain")
        .header("Status-Code", "200")
        .body(student.token.to_string())
        .unwrap()
}

pub(crate) async fn translate(token: String, map: HashMap<String, String>) -> Response<String> {
    let collection = db::generate_connection(&CONFIG).await;
    // Verify token, the header is like `Authorization: Basic dGVzdA==`
    let token = token.trim().split(" ").collect::<Vec<&str>>()[1].to_owned();
    let student = get_student(&collection, &token).await;
    if let Some(student) = student {
        db::add_one(&collection, student).await;
    } else {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", 403)
            .body("FORBIDDEN".to_owned())
            .unwrap();
    }

    Response::builder()
        .header("Content-Type", "application/json")
        .header("Status-Code", "200")
        .body("".to_owned())
        .unwrap()
}
