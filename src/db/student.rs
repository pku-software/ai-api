use mongodb::bson::{doc, Bson};
use mongodb::options::UpdateModifications;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct Student {
    pub id: String,
    pub num: i32,
    pub token: String,
}

impl Student {
    pub fn new(id: String) -> Student {
        Student {
            id,
            num: 0,
            token: Uuid::new_v4().to_string(),
        }
    }
}

impl Into<UpdateModifications> for Student {
    fn into(self) -> UpdateModifications {
        let doc = doc! {"id": &self.id, "num": &self.num, "token": &self.token};
        UpdateModifications::Document(doc)
    }
}

impl Into<Bson> for Student {
    fn into(self) -> Bson {
        Bson::Document(doc! {"id": &self.id, "num": &self.num, "token": &self.token})
    }
}
