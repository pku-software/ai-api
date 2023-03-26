use crate::openai::chat::chat;
use crate::CONFIG;
use serde_json::json;
#[tokio::test]
async fn test_chatgpt() {
    let dat = chat(&CONFIG.chat, "你好").await;
    assert!(dat["status"] == "ok");
}
