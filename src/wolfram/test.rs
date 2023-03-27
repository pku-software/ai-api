use image::EncodableLayout;

use crate::wolfram::wolfram;

#[tokio::test]
async fn test_wolfram() {
    let res = wolfram("What is the capital of China?".to_string()).await;
    assert!(res.is_ok());
    let pic = res.unwrap();
    let pic = image::load_from_memory_with_format(pic.as_bytes(), image::ImageFormat::Gif).unwrap();
    pic.save_with_format("~/pic/abc.bmp", image::ImageFormat::Bmp)
        .unwrap();
}
