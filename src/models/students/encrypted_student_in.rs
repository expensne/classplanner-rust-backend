use super::student_in::StudentIn;
use crate::encryption::encrypt::EncryptStudent;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedStudentIn {
    pub firstName: String,
    pub lastName: String,
    pub scores: String,
    pub nonce: String,
}

impl EncryptStudent for StudentIn {
    fn encrypt(&self, cipher: &ChaCha20Poly1305) -> EncryptedStudentIn {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let scores_bytes = serde_json::to_vec(&self.scores).unwrap();

        let first_name_encypted = cipher.encrypt(&nonce, self.firstName.as_ref()).unwrap();
        let last_name_encrypted = cipher.encrypt(&nonce, self.lastName.as_ref()).unwrap();
        let scores_encrypted = cipher.encrypt(&nonce, scores_bytes.as_ref()).unwrap();

        let first_name_encoded = hex::encode(first_name_encypted);
        let last_name_encoded = hex::encode(last_name_encrypted);
        let scores_encoded = hex::encode(scores_encrypted);
        let nonce_encoded = hex::encode(nonce);

        EncryptedStudentIn {
            firstName: first_name_encoded,
            lastName: last_name_encoded,
            scores: scores_encoded,
            nonce: nonce_encoded,
        }
    }
}
