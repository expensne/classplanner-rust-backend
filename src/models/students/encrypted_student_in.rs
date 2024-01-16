use super::{score::Score, student_in::StudentIn};
use crate::encryption::encrypt::Encrypt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedStudentIn {
    pub firstName: String,
    pub lastName: String,
    pub scores: HashMap<String, Score>,
    pub nonce: String,
}

impl Encrypt for StudentIn {
    fn encrypt(self, encryption_key: &str) -> EncryptedStudentIn {
        let encrypted_first_name = self.firstName + encryption_key; // TODO
        let encrypted_last_name = self.lastName + encryption_key; // TODO
        let nonce = "abcd".to_owned(); // TODO

        EncryptedStudentIn {
            firstName: encrypted_first_name,
            lastName: encrypted_last_name,
            scores: self.scores,
            nonce,
        }
    }
}
