use super::{exam_in::ExamIn, grading_scale::GradingScale};
use crate::encryption::encrypt::EncryptExam;
use chacha20poly1305::{
    aead::{Aead, OsRng},
    AeadCore, ChaCha20Poly1305,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedExamIn {
    pub name: String,
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
    pub nonce: String,
}

impl EncryptExam for ExamIn {
    fn encrypt(&self, cipher: &ChaCha20Poly1305) -> EncryptedExamIn {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);

        let encrypted_name = cipher.encrypt(&nonce, self.name.as_ref()).unwrap();

        let encrypted_name_encoded = hex::encode(encrypted_name);
        let nonce_encoded = hex::encode(nonce);

        EncryptedExamIn {
            name: encrypted_name_encoded,
            maxPoints: self.maxPoints,
            gradingScale: self.gradingScale.to_owned(),
            nonce: nonce_encoded,
        }
    }
}
