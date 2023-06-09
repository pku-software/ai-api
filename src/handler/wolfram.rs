use super::utils::*;
use crate::handler::utils;
use base64::Engine;
use image::{self, EncodableLayout};
use serde_json::json;
use std::{collections::HashMap, io};
use warp::http::Response;

pub(crate) async fn wolfram(token: String, map: HashMap<String, String>) -> Response<String> {
    if check_token(&token, crate::db::log::LogType::MATH)
        .await
        .is_err()
    {
        return token_error();
    }

    let input = match get_string_from_map(&map, "input") {
        Ok(input) => input,
        Err(res) => return res,
    };

    let ans = crate::wolfram::wolfram(input.to_owned()).await;

    if ans.is_err() {
        warn!("Wolfram Alpha API error {}", ans.err().unwrap());
        return utils::network_error();
    }

    let gif_image =
        image::load_from_memory_with_format(ans.unwrap().as_bytes(), image::ImageFormat::Gif);

    if gif_image.is_err() {
        warn!(
            "Wolfram Alpha image decode error: {:}",
            &gif_image.err().unwrap().to_string().clone()
        );
        return utils::image_decode_error();
    }

    let gif_image = gif_image.unwrap();
    // convert to base64 png
    let mut pic = io::Cursor::new(Vec::new());
    gif_image
        .write_to(&mut pic, image::ImageOutputFormat::Png)
        .unwrap();

    let base64_pic = base64::engine::general_purpose::STANDARD_NO_PAD.encode(pic.get_ref());
    Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(
            json!(
                {
                    "status": "ok",
                    "image": format!("data:image/png;base64,{}", base64_pic)
                }
            )
            .to_string(),
        )
        .unwrap()
}
