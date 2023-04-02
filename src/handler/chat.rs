use super::utils::*;
use crate::CONFIG;
use std::collections::HashMap;
use warp::http::Response;

pub(crate) async fn chat(token: String, map: HashMap<String, String>) -> Response<String> {
    if check_token(&token, crate::db::log::LogType::CHAT)
        .await
        .is_err()
    {
        return token_error();
    }

    let prompt = match get_string_from_map(&map, "prompt") {
        Ok(promot) => promot,
        Err(res) => return res,
    };

    let prompt = prompt.clone().trim().to_string();

    let ans = crate::openai::chat(&CONFIG.chat, &prompt).await;
    Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(ans.to_string())
        .unwrap()
}
