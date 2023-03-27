use crate::translate::translate;
use serde_json::json;

#[tokio::test]
async fn test_baidu_translate() {
    let dat = translate("en", "zh", "apple").await; // should equal to "你好，世界！"
    let ans = json!({
        "status": "ok",
        "text": "苹果",
    });
    assert_eq!(dat, ans);
}

#[tokio::test]
async fn test_translate_null_text() {
    let dat = translate("en", "zh", "").await;
    let ans = json!({
        "status": "failed",
        "text": "Text is empty",
    });
    assert_eq!(dat, ans);
}
