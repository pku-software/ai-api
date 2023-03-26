use crate::openai::chat::chat;
use crate::openai::draw::draw;
use crate::CONFIG;
use serde_json::json;
#[tokio::test]
async fn test_chatgpt() {
    let dat = chat(&CONFIG.chat, "你好").await;
    assert!(dat["status"] == "ok");
}

#[tokio::test]
async fn test_draw() {
    let dat = draw("apple".to_owned(), 256, 256).await;
    assert!(dat["status"] == "ok");
}
