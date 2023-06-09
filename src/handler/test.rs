use crate::config::Config;
use crate::db;
use crate::db::student::Student;
use crate::CONFIG;
use mongodb::{options::ClientOptions, Client, Collection};
use serde_json::json;
use std::collections::HashMap;

async fn generate_db_connection() -> Collection<Student> {
    let config = Config::new();
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("test").collection::<Student>("student");
    collection
}

#[tokio::test]
async fn test_translate_handler() {
    let prod_db = db::generate_connection(&CONFIG).await;
    let student = db::get_student(&prod_db, "test").await;
    assert!(student.is_some());
    let student = student.unwrap();

    let authorization_header = format!("Bearer {}", student.token);

    // test get translate
    let mut translate_map = HashMap::new();
    translate_map.insert("text".to_owned(), "hello".to_owned());

    let res = crate::handler::translate(authorization_header, translate_map).await;
    assert_eq!(res.status(), 200);

    assert_eq!(
        res.body().to_owned(),
        json!(
            {
                "status": "ok",
                "text": "你好"
            }
        )
        .to_string()
    );

    // test get translate with wrong token
    let authorization_header = "Bearer wrongtoken".to_owned();
    let mut translate_map = HashMap::new();
    translate_map.insert("text".to_owned(), "hello".to_owned());

    let res = crate::handler::translate(authorization_header, translate_map).await;
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_draw_handler() {
    let prod_db = db::generate_connection(&CONFIG).await;
    let student = db::get_student(&prod_db, "test").await;
    assert!(student.is_some());
    let student = student.unwrap();

    let authorization_header = format!("Bearer {}", student.token);

    // test get draw
    let mut draw_map = HashMap::new();
    draw_map.insert("prompt".to_owned(), "apple".to_owned());
    draw_map.insert("kind".to_owned(), "1".to_owned());

    let res = crate::handler::draw(authorization_header.clone(), draw_map).await;
    assert_eq!(res.status(), 200);

    //test wrong kind
    let mut draw_map = HashMap::new();
    draw_map.insert("prompt".to_owned(), "apple".to_owned());
    draw_map.insert("kind".to_owned(), "6".to_owned());

    let res = crate::handler::draw(authorization_header.clone(), draw_map).await;
    assert_eq!(res.status(), 400);

    // test get draw with wrong token
    let authorization_header = "Bearer wrongtoken".to_owned();
    let mut draw_map = HashMap::new();
    draw_map.insert("prompt".to_owned(), "apple".to_owned());
    draw_map.insert("kind".to_owned(), "1".to_owned());

    let res = crate::handler::draw(authorization_header.clone(), draw_map).await;
    assert_eq!(res.status(), 401);
}

#[tokio::test]
async fn test_wolframe_handler() {
    let prod_db = db::generate_connection(&CONFIG).await;
    let student = db::get_student(&prod_db, "test").await;
    assert!(student.is_some());
    let student = student.unwrap();

    let authorization_header = format!("Bearer {}", student.token);

    // test get draw
    let mut wolframe_map = HashMap::new();
    wolframe_map.insert("input".to_owned(), "apple".to_owned());

    let res = crate::handler::wolfram(authorization_header.clone(), wolframe_map).await;
    assert_eq!(res.status(), 200);
}
