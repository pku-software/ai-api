use serde_json::json;

use crate::CONFIG;

const IMAGE_BASEURI: &str = "https://api.openai.com/v1/images/generations";

pub(crate) async fn draw(prompt: String, height: i32, width: i32) -> serde_json::Value {
    let request_json = json!(
        {
            "prompt": prompt,
            "n":1,
            "size": format!("{}x{}", height, width),
            "response_format": "b64_json"
        }
    );
    let client = reqwest::Client::new();
    let res = client
        .post(IMAGE_BASEURI)
        .bearer_auth(&CONFIG.chat.token)
        .header("Content-Type", "application/json")
        .body(request_json.to_string())
        .send()
        .await;
    let res = match res {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to send request: {}", err);
            return json!({
                "status": "failed",
                "error": "Network error",
            });
        }
    };
    if res.status() != 200 {
        return json!({
            "status": "failed",
            "error": "Network error",
        });
    }
    let res = match res.text().await {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to get response: {}", err);
            return json!({
                "status": "failed",
                "error": "Cannot get response",
            });
        }
    };
    let res = match serde_json::from_str::<serde_json::Value>(&res) {
        Ok(res) => res,
        Err(err) => {
            error!("Failed to parse response: {}", err);
            return json!({
                "status": "failed",
                "error": "JSON parse error",
            });
        }
    };

    let data = match res["data"].as_array() {
        Some(data) => data,
        None => {
            error!("Failed to parse response: data is not an array");
            return json!({
                "status": "failed",
                "error": "JSON parse error",
            });
        }
    };
    let data = match data.get(0) {
        Some(data) => data,
        None => {
            error!("Failed to parse response: data is empty");
            return json!({
                "status": "failed",
                "error": "JSON parse error",
            });
        }
    };
    let data = match data["b64_json"].as_str() {
        Some(data) => data,
        None => {
            error!("Failed to parse response: data is not a string");
            return json!({
                "status": "failed",
                "error": "JSON parse error",
            });
        }
    };

    json!(
        {
            "status": "ok",
            "decoded_image": data,
        }
    )
}
