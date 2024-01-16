use super::{score::Score, student_in::StudentIn};
use crate::encryption::encrypt::Encrypt;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305,
};
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
    fn encrypt(self, cipher: &ChaCha20Poly1305) -> EncryptedStudentIn {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let encrypted_first_name = cipher.encrypt(&nonce, self.firstName.as_ref()).unwrap();
        let encrypted_last_name = cipher.encrypt(&nonce, self.lastName.as_ref()).unwrap();

        let encrypted_first_name_encoded = hex::encode(encrypted_first_name);
        let encrypted_last_name_encoded = hex::encode(encrypted_last_name);
        let nonce_encoded = hex::encode(nonce);

        EncryptedStudentIn {
            firstName: encrypted_first_name_encoded,
            lastName: encrypted_last_name_encoded,
            scores: self.scores,
            nonce: nonce_encoded,
        }
    }
}
