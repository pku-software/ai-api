use crate::config::Config;
use crate::db::*;
use mongodb::{Client, Collection, Database};

async fn generate_connection() -> Collection<Student> {
    let config = Config::new();
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("test").collection::<Student>("student");
    collection
}

async fn test_db_connection() {
    let config = Config::new();
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options);
    assert!(client.is_ok());
    // drop the 'test' db here
    let db = client.unwrap().database("test");
    let res = db.drop(None).await;
    assert!(res.is_ok());
}

async fn test_db_insert() {
    let collection = generate_connection().await;
    let student = Student::new("test".to_string());
    insert_student(&collection, &student).await;
    let student = get_student(&collection, "test").await.unwrap();
    assert_eq!(student.id, "test");
    assert_eq!(student.num, 0);
    assert_eq!(student.token.len(), 36);
    let student = Student::new("test2".to_string());
    insert_student(&collection, &student).await;
}

async fn test_db_update() {
    let collection = generate_connection().await;
    let student = get_student(&collection, "test").await.unwrap();
    let mut student = student;
    student.num = 9;
    update_student(&collection, &student).await;
    let student = get_student(&collection, "test").await.unwrap();
    assert_eq!(student.id, "test");
    assert_eq!(student.num, 9);
    assert_eq!(student.token.len(), 36);
}

async fn test_db_get_student_by_token() {
    let collection = generate_connection().await;
    let student = get_student(&collection, "test").await.unwrap();
    let student = get_student_by_token(&collection, &student.token)
        .await
        .unwrap();
    assert_eq!(student.id, "test");
}

async fn test_db_delete() {
    let collection = generate_connection().await;
    delete_student(&collection, "test").await;
    let student = get_student(&collection, "test").await;
    assert_eq!(student, None);
}

async fn test_db_get_all() {
    let collection = generate_connection().await;
    let students = get_all_students(&collection).await;
    assert_eq!(students.len(), 1);
    assert_eq!(students[0].id, "test2");
    assert_eq!(students[0].num, 0);
    assert_eq!(students[0].token.len(), 36);
    delete_student(&collection, "test2").await;
}

#[tokio::test]
async fn test_db() {
    test_db_connection().await;
    test_db_insert().await;
    test_db_update().await;
    test_db_get_student_by_token().await;
    test_db_delete().await;
    test_db_get_all().await;
}
