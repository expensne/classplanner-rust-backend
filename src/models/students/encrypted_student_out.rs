use super::{encrypted_student_in::EncryptedStudentIn, score::Score, student_out::StudentOut};
use crate::encryption::decrypt::DecryptStudent;
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Nonce};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedStudentOut {
    pub id: String,
    pub firstName: String,
    pub lastName: String,
    pub scores: String,
    pub nonce: String,
}

impl EncryptedStudentOut {
    fn try_decrypt(
        &self,
        cipher: &ChaCha20Poly1305,
    ) -> Result<StudentOut, Box<dyn std::error::Error>> {
        let first_name_decoded = hex::decode(self.firstName.to_owned())?;
        let last_name_decoded = hex::decode(self.lastName.to_owned())?;
        let scores_decoded = hex::decode(self.scores.to_owned())?;
        let nonce_decoded = hex::decode(self.nonce.to_owned())?;

        let nonce = Nonce::from_slice(nonce_decoded.as_ref());

        let first_name_decrypted = cipher
            .decrypt(&nonce, first_name_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let last_name_decrypted = cipher
            .decrypt(&nonce, last_name_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let scores_decrypted = cipher
            .decrypt(&nonce, scores_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let first_name = String::from_utf8(first_name_decrypted)?;
        let last_name = String::from_utf8(last_name_decrypted)?;
        let scores: HashMap<String, Score> = serde_json::from_slice(scores_decrypted.as_ref())?;

        Ok(StudentOut {
            id: self.id.to_owned(),
            firstName: first_name,
            lastName: last_name,
            scores: scores,
        })
    }
}

impl DecryptStudent for EncryptedStudentOut {
    fn decrypt(&self, cipher: &ChaCha20Poly1305) -> StudentOut {
        match self.try_decrypt(cipher) {
            Ok(student) => student,
            Err(_) => {
                tracing::warn!("Failed to decrypt student {:?}", self);
                StudentOut {
                    id: self.id.to_owned(),
                    firstName: "Failed to decrypt".to_string(),
                    lastName: "Failed to decrypt".to_string(),
                    scores: HashMap::new(),
                }
            }
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
        let scores = doc.get_str("scores").unwrap().to_string();
        let nonce = doc.get_str("nonce").unwrap().to_string();

        Self {
            id,
            firstName: first_name,
            lastName: last_name,
            scores,
            nonce,
        }
    }
}
