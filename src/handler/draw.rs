use super::utils::*;
use std::collections::HashMap;
use warp::http::Response;

pub(crate) async fn draw(token: String, map: HashMap<String, String>) -> Response<String> {
    if check_token(&token).await.is_err() {
        return token_error();
    }

    let prompt = match get_string_from_map(&map, "prompt") {
        Ok(prompt) => prompt,
        Err(res) => return res,
    };
    let prompt = prompt.clone().trim().to_string();

    let kind = match get_i32_from_map(&map, "kind") {
        Ok(kind) => kind,
        Err(res) => return res,
    };

    if kind <= 0 || kind > 3 {
        return bad_request();
    }

    let ans = crate::openai::draw(prompt, kind).await;

    Response::builder()
        .header("Content-Type", "text/plain")
        .status(200)
        .body(ans.to_string())
        .unwrap()
}
