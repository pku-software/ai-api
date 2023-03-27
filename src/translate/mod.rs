mod test;

use crate::CONFIG;
use rand::{self, Rng};
use serde_json::json;
use warp::body::json;

const BAIDU_TRANSLATE_URL: &str = "https://fanyi-api.baidu.com/api/trans/vip/translate";

fn generate_salt() -> String {
    let mut rng = rand::thread_rng();
    let salt: u32 = rng.gen();
    salt.to_string()
}

async fn baidu_translate(from: &str, to: &str, text: &str) -> Result<serde_json::Value, String> {
    let salt = generate_salt();
    let sign_string = format!(
        "{}{}{}{}",
        CONFIG.translate.appid, text, salt, CONFIG.translate.secret
    );
    // generate md5, and convert to lowercase string
    let sign = format!("{:x}", md5::compute(sign_string)).to_lowercase();

    let query_string = format!(
        "q={}&from={}&to={}&appid={}&salt={}&sign={}",
        text, from, to, CONFIG.translate.appid, salt, sign
    );

    let url = format!("{}?{}", BAIDU_TRANSLATE_URL, query_string);

    let resp = match reqwest::get(&url).await {
        Ok(resp) => resp,
        Err(err) => {
            error!("Failed to get response from Baidu translate: {}", err);
            return Err(format!("Failed to get response from Baidu translate",));
        }
    };

    let json: serde_json::Value = match resp.json().await {
        Ok(json) => json,
        Err(err) => {
            error!("Failed to parse response from Baidu translate: {}", err);
            return Err(format!("Failed to parse response from Baidu translate"));
        }
    };
    Ok(json)
}

pub(crate) async fn translate(from: &str, to: &str, text: &str) -> serde_json::Value {
    if text.len() == 0 {
        return json!({
            "status": "failed",
            "text": "Text is empty",
        });
    }
    let text = baidu_translate(from, to, text).await;
    if text.is_ok() {
        return json!({
            "status": "ok",
            "text": text.unwrap()["trans_result"][0]["dst"].as_str(),
        });
    } else {
        return json!({
            "status": "failed",
            "text": text.err().unwrap(),
        });
    }
}
