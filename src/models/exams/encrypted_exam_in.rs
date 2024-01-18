use super::exam_in::ExamIn;
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
    pub maxPoints: String,
    pub gradingScale: String,
    pub nonce: String,
}

impl EncryptExam for ExamIn {
    fn encrypt(&self, cipher: &ChaCha20Poly1305) -> EncryptedExamIn {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let max_points_bytes = self.maxPoints.to_ne_bytes();
        let grading_scale_bytes = serde_json::to_vec(&self.gradingScale).unwrap();

        let name_encrypted = cipher.encrypt(&nonce, self.name.as_ref()).unwrap();
        let max_points_encrypted = cipher.encrypt(&nonce, max_points_bytes.as_ref()).unwrap();
        let grading_scale_encrypted = cipher
            .encrypt(&nonce, grading_scale_bytes.as_ref())
            .unwrap();

        let name_encoded = hex::encode(name_encrypted);
        let max_points_encoded = hex::encode(max_points_encrypted);
        let grading_scale_encoded = hex::encode(grading_scale_encrypted);
        let nonce_encoded = hex::encode(nonce);

        EncryptedExamIn {
            name: name_encoded,
            maxPoints: max_points_encoded,
            gradingScale: grading_scale_encoded,
            nonce: nonce_encoded,
        }
    }
}
