use super::utils::*;
use crate::translate;
use crate::CONFIG;
use std::collections::HashMap;
use warp::http::Response;

pub(crate) async fn translate(token: String, map: HashMap<String, String>) -> Response<String> {
    if check_token(&token, crate::db::log::LogType::TRANSLATE)
        .await
        .is_err()
    {
        return token_error();
    }

    let text = match get_string_from_map(&map, "text") {
        Ok(text) => text,
        Err(res) => return res,
    };
    let text = text.clone().trim().to_string();

    let from = map.get("from").unwrap_or(&CONFIG.translate.source);

    let to = map.get("to").unwrap_or(&CONFIG.translate.target);

    let result = translate::translate(from, to, &text).await.to_string();
    Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(result)
        .unwrap()
}
