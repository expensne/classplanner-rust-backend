use std::collections::HashMap;

use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Score {
    pub pointsScored: f64,
    pub isPostscript: bool,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct Student {
    pub firstName: String,
    pub lastName: String,
    pub scores: HashMap<String, Score>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct StudentResponse {
    pub id: String,
    pub firstName: String,
    pub lastName: String,
    pub scores: HashMap<String, Score>,
}

impl From<Student> for StudentResponse {
    fn from(student: Student) -> Self {
        Self {
            id: "".to_owned(),
            firstName: student.firstName,
            lastName: student.lastName,
            scores: student.scores,
        }
    }
}

impl From<Document> for StudentResponse {
    fn from(doc: Document) -> Self {
        let id = doc.get_object_id("_id").unwrap().to_hex();
        let first_name = doc.get_str("firstName").unwrap().to_string();
        let last_name = doc.get_str("lastName").unwrap().to_string();
        let scores = doc
            .get_document("scores")
            .unwrap()
            .into_iter()
            .map(|(key, value)| {
                let value_doc = value.as_document().unwrap();
                (
                    key.to_string(),
                    Score {
                        pointsScored: value_doc.get_f64("pointsScored").unwrap(),
                        isPostscript: value_doc.get_bool("isPostscript").unwrap(),
                    },
                )
            })
            .collect();
        Self {
            id,
            firstName: first_name,
            lastName: last_name,
            scores,
        }
    }
}
