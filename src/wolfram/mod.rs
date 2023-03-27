mod test;

use crate::CONFIG;
use std::collections::HashMap;

const WOLFRAM_BASEURL: &str = "https://api.wolframalpha.com/v1/simple";

pub(crate) async fn wolfram(query: String) -> Result<String, String> {
    let client = reqwest::Client::new();
    let res = client
        .get(WOLFRAM_BASEURL)
        .query(&[("appid", &CONFIG.math.token), ("i", &query)])
        .send()
        .await;
    let res = match res {
        Ok(res) => res,
        Err(_) => return Err("Network error".to_string()),
    };

    if res.status() != 200 {
        return Err("WolframAlpha API error".to_string());
    }

    let res = res.text().await.unwrap();
    Ok(res)
}
