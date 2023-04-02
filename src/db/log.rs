use mongodb::bson::{doc, oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub(crate) enum LogType {
    TOKEN = 0,
    TRANSLATE,
    CHAT,
    DRAW,
    MATH,
}

impl Into<mongodb::bson::Bson> for LogType {
    fn into(self) -> mongodb::bson::Bson {
        mongodb::bson::Bson::Int32(self as i32)
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct Log {
    #[serde(rename = "_id")]
    id: ObjectId,
    student_id: String,
    time: DateTime,
    log_type: LogType,
}

impl Log {
    pub(crate) fn new(student_id: String, log_type: LogType) -> Log {
        Log {
            id: ObjectId::new(),
            student_id,
            time: DateTime::now(),
            log_type,
        }
    }

    pub(crate) fn to_doc(&self) -> mongodb::bson::Document {
        doc! {
          "_id": &self.id,
          "student_id": &self.student_id,
          "time": &self.time,
          "log_type": &self.log_type,
        }
    }
}
