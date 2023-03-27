use crate::wolfram::wolfram;

#[tokio::test]
async fn test_wolfram() {
    let res = wolfram("What is the capital of China?".to_string()).await;
    assert!(res.is_ok());
}
