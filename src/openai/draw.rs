use serde_json::json;

use crate::CONFIG;

const IMAGE_BASEURI: &str = "https://api.openai.com/v1/images/generations";

pub async fn draw(prompt: String, kind: i32) -> serde_json::Value {
    let size = match kind {
        1 => "256x256",
        2 => "512x512",
        3 => "1024x1024",
        _ => "256x256",
    };

    let request_json = json!(
        {
            "prompt": prompt,
            "n":1,
            "size": size,
            "response_format": "b64_json"
        }
    );
    let client = reqwest::Client::new();
    let res = client
        .post(IMAGE_BASEURI)
        .bearer_auth(&CONFIG.draw.token)
        .header("Content-Type", "application/json")
        .body(request_json.to_string())
        .send()
        .await;
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to send request: {}", err);
            return super::error::network_error();
        }
    };
    if res.status() != 200 {
        return super::error::network_error();
    }
    let res = match res.text().await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to get response: {}", err);
            return super::error::network_error();
        }
    };
    let res = match serde_json::from_str::<serde_json::Value>(&res) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to parse response: {}", err);
            super::error::parse_json_error()
        }
    };

    let data = match res["data"].as_array() {
        Some(data) => data,
        None => return super::error::parse_json_error(),
    };
    let data = match data.get(0) {
        Some(data) => data.clone(),
        None => {
            error!("Failed to parse response: data is empty");
            super::error::parse_json_error()
        }
    };
    let data = match data["b64_json"].as_str() {
        Some(data) => data,
        None => {
            error!("Failed to parse response: data is not a string");
            return super::error::parse_json_error();
        }
    };

    json!(
        {
            "status": "ok",
            "decoded_image": data,
        }
    )
}
