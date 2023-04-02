pub(crate) mod log;
pub(crate) mod student;
mod test;

use csv;
use futures::StreamExt;
use mongodb::{bson::doc, options::ClientOptions, Client, Collection, Database};
use student::Student;

use crate::{config::Config, CONFIG};

use self::log::{Log, LogType};

pub(crate) async fn init(config: &Config) {
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("ai").collection::<Student>("student");
    let mut rdr = csv::Reader::from_path(&config.user_csv).unwrap();
    for id in rdr.records() {
        let id = id.unwrap()[0].to_string();
        if get_student(&collection, &id).await.is_some() {
            continue;
        }
        let student = Student::new(id);
        insert_student(&collection, &student).await;
    }
}

pub(crate) async fn generate_connection(config: &Config) -> Collection<Student> {
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("ai").collection::<Student>("student");
    collection
}

pub(crate) async fn generate_log_connection(config: &Config) -> Collection<Log> {
    let client_options = ClientOptions::parse(&config.mongo_uri).await.unwrap();
    let client = Client::with_options(client_options).unwrap();
    let collection = client.database("ai").collection::<Log>("log");
    collection
}

pub(crate) async fn get_student(collection: &Collection<Student>, id: &str) -> Option<Student> {
    let doc = doc! {"id": id};
    let student = collection.find_one(doc, None).await.unwrap();
    student
}

pub(crate) async fn get_student_by_token(
    collection: &Collection<Student>,
    token: &str,
) -> Option<Student> {
    let doc = doc! {"token": token};
    let student = collection.find_one(doc, None).await.unwrap();
    student
}

pub(crate) async fn update_student(collection: &Collection<Student>, student: &Student) {
    let filter = doc! {"id": &student.id};
    let update = doc! {"$set": student};
    collection.update_one(filter, update, None).await.unwrap();
}

pub(crate) async fn insert_student(collection: &Collection<Student>, student: &Student) {
    collection.insert_one(student, None).await.unwrap();
}

pub(crate) async fn delete_student(collection: &Collection<Student>, id: &str) {
    let doc = doc! {"id": id};
    collection.delete_one(doc, None).await.unwrap();
}

pub(crate) async fn get_all_students(collection: &Collection<Student>) -> Vec<Student> {
    let mut cursor = collection.find(None, None).await.unwrap();
    let mut students = Vec::new();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(student) => students.push(student),
            Err(e) => error!("Failed to get student: {}", e),
        }
    }
    students
}

pub(crate) async fn add_one(collection: &Collection<Student>, student: Student, log_type: LogType) {
    add_log(student, log_type).await;
}

pub(crate) async fn add_log(student: Student, log_type: LogType) {
    let log_collection = generate_log_connection(&CONFIG).await;
    let log = Log::new(student.id.clone(), log_type);
    log_collection.insert_one(log, None).await.unwrap();
}
