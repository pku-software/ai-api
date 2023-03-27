use super::utils::*;
use crate::db;
use crate::CONFIG;
use std::collections::HashMap;
use warp::http::Response;

pub(crate) async fn get_token(map: HashMap<String, String>) -> Response<String> {
    let collection = db::generate_connection(&CONFIG).await;
    let id = match get_string_from_map(&map, "id") {
        Ok(id) => id,
        Err(res) => return res,
    };

    let student = db::get_student(&collection, id).await;
    if student.is_none() {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .status(404)
            .body("STUDENT NOT FOUND".to_owned())
            .unwrap();
    }
    let mut student = student.unwrap();

    if student.used {
        return Response::builder()
            .header("Content-Type", "text/plain")
            .status(403)
            .body(
                "The id has been used. If you have lost the token, please contact the TA."
                    .to_owned(),
            )
            .unwrap();
    }

    student.used = true;
    db::update_student(&collection, &student).await;
    info!("student {} get token", &student.id);
    Response::builder()
        .header("Content-Type", "text/plain")
        .status(200)
        .body(student.token.to_string())
        .unwrap()
}
