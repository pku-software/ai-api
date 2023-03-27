use crate::config::Config;
use crate::db;
use crate::db::student::Student;
use crate::CONFIG;
use mongodb::{options::ClientOptions, Client, Collection};
use std::collections::HashMap;

async fn generate_db_connection() -> Collection<Student> {
    let config = Config::new();
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("test").collection::<Student>("student");
    collection
}

#[tokio::test]
async fn test_get_token_handler() {
    let prod_db = db::generate_connection(&CONFIG).await;
    let student = db::get_student(&prod_db, "test").await;
    assert!(student.is_some());
    let student = student.unwrap();

    // test get token
    let mut token_map = HashMap::new();
    token_map.insert("id".to_owned(), "test".to_owned());
    token_map.insert("password".to_owned(), "42".to_owned());

    let res = crate::handler::get_token(token_map).await;
    assert_eq!(res.status(), 200);
    let token = res.body();
    assert_eq!(token.to_owned(), student.token);

    // test get with wrong id or password
    let mut token_map = HashMap::new();
    token_map.insert("id".to_owned(), "test".to_owned());
    token_map.insert("password".to_owned(), "43".to_owned());

    let res = crate::handler::get_token(token_map).await;
    assert_eq!(res.status(), 403);

    let mut token_map = HashMap::new();
    token_map.insert("id".to_owned(), "neverhappen".to_owned());
    token_map.insert("password".to_owned(), "42".to_owned());

    let res = crate::handler::get_token(token_map).await;
    assert_eq!(res.status(), 404);
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
