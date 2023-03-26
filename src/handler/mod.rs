use crate::db;
use crate::db::student::Student;
use crate::db::*;
use crate::openai::{chat, draw};
use crate::translate;
use crate::CONFIG;
use std::collections::HashMap;
use warp::http::Response;

async fn check_token(token: &String) -> Result<(), String> {
    let collection = db::generate_connection(&CONFIG).await;
    let token = token.trim().split(" ").collect::<Vec<&str>>()[1].to_owned();
    let student = get_student(&collection, &token).await;
    if let Some(student) = student {
        db::add_one(&collection, student).await;
        return Ok(());
    } else {
        return Err(format!("Invalid token {:}", token));
    }
}

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
    let token_result = check_token(&token).await;
    if let Err(err) = token_result {
        warn!("Invalid token: {}", err);
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "403")
            .body(err)
            .unwrap();
    }

    let text = match map.get("text") {
        Some(text) => text,
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .header("Status-Code", "400")
                .body("BAD REQUEST".to_owned())
                .unwrap();
        }
    };
    let text = text.clone().trim().to_string();

    let from = match map.get("from") {
        Some(from) => from,
        None => &CONFIG.translate.source,
    };

    let to = match map.get("to") {
        Some(to) => to,
        None => &CONFIG.translate.target,
    };

    let result = translate::translate(&text, from, to).await.to_string();
    Response::builder()
        .header("Content-Type", "text/plain")
        .header("Status-Code", "200")
        .body(result)
        .unwrap()
}

pub(crate) async fn chat(token: String, map: HashMap<String, String>) -> Response<String> {
    let token_result = check_token(&token).await;
    if let Err(err) = token_result {
        warn!("Invalid token: {}", err);
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "403")
            .body(err)
            .unwrap();
    }

    let prompt = match map.get("prompt") {
        Some(prompt) => prompt,
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .header("Status-Code", "400")
                .body("BAD REQUEST".to_owned())
                .unwrap();
        }
    };

    let prompt = prompt.clone().trim().to_string();

    let ans = chat::chat(&CONFIG.chat, &prompt).await;
    Response::builder()
        .header("Content-Type", "text/plain")
        .header("Status-Code", "200")
        .body(ans.to_string())
        .unwrap()
}

async fn draw(token: String, map: HashMap<String, String>) -> Response<String> {
    let token_result = check_token(&token).await;
    if let Err(err) = token_result {
        warn!("Invalid token: {}", err);
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "403")
            .body(err)
            .unwrap();
    }

    let prompt = match map.get("prompt") {
        Some(prompt) => prompt,
        None => {
            return Response::builder()
                .header("Content-Type", "text/plain")
                .header("Status-Code", "400")
                .body("BAD REQUEST".to_owned())
                .unwrap();
        }
    };

    let prompt = prompt.clone().trim().to_string();

    let height = map.get("height");

    let height = match height {
        Some(height) => height.parse::<i32>().unwrap(),
        None => CONFIG.draw.height,
    };

    let width = map.get("width");

    let width = match width {
        Some(width) => width.parse::<i32>().unwrap(),
        None => CONFIG.draw.width,
    };

    if width != height {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "400")
            .body("BAD REQUEST".to_owned())
            .unwrap();
    }

    if width != 256 && width != 512 && width != 1024 {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .header("Status-Code", "400")
            .body("BAD REQUEST".to_owned())
            .unwrap();
    }

    let ans = draw::draw(prompt, height, width).await;

    Response::builder()
        .header("Content-Type", "text/plain")
        .header("Status-Code", "200")
        .body(ans.to_string())
        .unwrap()
}
