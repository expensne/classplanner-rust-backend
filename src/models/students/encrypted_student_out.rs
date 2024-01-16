use super::{encrypted_student_in::EncryptedStudentIn, score::Score, student_out::StudentOut};
use crate::encryption::decrypt::Decrypt;
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedStudentOut {
    pub id: String,
    pub firstName: String,
    pub lastName: String,
    pub scores: HashMap<String, Score>,
    pub nonce: String,
}

impl Decrypt for EncryptedStudentOut {
    fn decrypt(self, key: &str) -> StudentOut {
        let decrypted_first_name = self.firstName.replace(key, ""); // TODO
        let decrypted_last_name = self.lastName.replace(key, ""); // TODO

        StudentOut {
            id: self.id,
            firstName: decrypted_first_name,
            lastName: decrypted_last_name,
            scores: self.scores,
        }
    }
}

impl From<EncryptedStudentIn> for EncryptedStudentOut {
    fn from(student: EncryptedStudentIn) -> Self {
        Self {
            id: "".to_owned(),
            firstName: student.firstName,
            lastName: student.lastName,
            scores: student.scores,
            nonce: student.nonce,
        }
    }
}

impl From<Document> for EncryptedStudentOut {
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
        let nonce = match doc.get_str("nonce") {
            Ok(nonce) => nonce.to_string(),
            Err(_) => "missing".to_string(),
        };


        Self {
            id,
            firstName: first_name,
            lastName: last_name,
            scores,
            nonce,
        }
    }
}
