use super::utils::*;
use crate::{handler::utils, CONFIG};
use base64::Engine;
use image;
use serde_json::json;
use std::{collections::HashMap, io};
use warp::{body::json, http::Response};

pub(crate) async fn wolfram(token: String, map: HashMap<String, String>) -> Response<String> {
    if check_token(&token).await.is_err() {
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
            gif_image.err().unwrap()
        );
        return utils::image_decode_error();
    }

    let gif_image = gif_image.unwrap();
    // convert to base64 bmp
    let mut bmp = io::Cursor::new(Vec::new());
    gif_image
        .write_to(&mut bmp, image::ImageOutputFormat::Bmp)
        .unwrap();

    let base64_bmp = base64::engine::general_purpose::STANDARD_NO_PAD.encode(bmp.get_ref());

    Response::builder()
        .header("Content-Type", "application/json")
        .status(200)
        .body(
            json!(
                {
                    "status": "ok",
                    "decoded_image": base64_bmp
                }
            )
            .to_string(),
        )
        .unwrap()
}
