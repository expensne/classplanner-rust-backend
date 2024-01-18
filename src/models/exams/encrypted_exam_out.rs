use super::{encrypted_exam_in::EncryptedExamIn, exam_out::ExamOut, grading_scale::GradingScale};
use crate::encryption::decrypt::DecryptExam;
use chacha20poly1305::{aead::Aead, ChaCha20Poly1305, Nonce};
use mongodb::bson::Document;
use serde::{Deserialize, Serialize};

#[derive(Clone, Deserialize, Serialize, Debug)]
#[allow(non_snake_case)]
pub struct EncryptedExamOut {
    pub id: String,
    pub name: String,
    pub maxPoints: String,
    pub gradingScale: String,
    pub nonce: String,
}

impl EncryptedExamOut {
    fn try_decrypt(
        &self,
        cipher: &ChaCha20Poly1305,
    ) -> Result<ExamOut, Box<dyn std::error::Error>> {
        let name_decoded = hex::decode(self.name.to_owned())?;
        let max_points_decoded = hex::decode(self.maxPoints.to_owned())?;
        let grading_scale_decoded = hex::decode(self.gradingScale.to_owned())?;
        let nonce_decoded = hex::decode(self.nonce.to_owned())?;

        let nonce = Nonce::from_slice(&nonce_decoded);

        let name_decrypted = cipher
            .decrypt(&nonce, name_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let max_points_decrypted = cipher
            .decrypt(&nonce, max_points_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;
        let grading_scale_decrypted = cipher
            .decrypt(&nonce, grading_scale_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let name = String::from_utf8(name_decrypted)?;
        let max_points = f64::from_ne_bytes(max_points_decrypted.try_into().unwrap());
        let grading_scale: GradingScale = serde_json::from_slice(&grading_scale_decrypted)?;

        Ok(ExamOut {
            id: self.id.to_owned(),
            name,
            maxPoints: max_points,
            gradingScale: grading_scale,
        })
    }
}

impl DecryptExam for EncryptedExamOut {
    fn decrypt(&self, cipher: &ChaCha20Poly1305) -> ExamOut {
        match self.try_decrypt(cipher) {
            Ok(exam) => exam,
            Err(_) => {
                tracing::warn!("Failed to decrypt exam {:?}", self);
                ExamOut {
                    id: self.id.to_owned(),
                    name: "Failed to decrypt".to_string(),
                    maxPoints: -1.0,
                    gradingScale: GradingScale {
                        A: -1.0,
                        B: -1.0,
                        C: -1.0,
                        D: -1.0,
                        E: -1.0,
                        F: -1.0,
                    },
                }
            }
        }
    }
}

impl From<EncryptedExamIn> for EncryptedExamOut {
    fn from(exam: EncryptedExamIn) -> Self {
        Self {
            id: "".to_owned(),
            name: exam.name,
            maxPoints: exam.maxPoints,
            gradingScale: exam.gradingScale,
            nonce: exam.nonce,
        }
    }
}

impl From<Document> for EncryptedExamOut {
    fn from(doc: Document) -> Self {
        let id = doc.get_object_id("_id").unwrap().to_hex();
        let name = doc.get_str("name").unwrap().to_string();
        let max_points = doc.get_str("maxPoints").unwrap().to_string();
        let grading_scale = doc.get_str("gradingScale").unwrap().to_string();
        let nonce = doc.get_str("nonce").unwrap().to_string();

        Self {
            id,
            name,
            maxPoints: max_points,
            gradingScale: grading_scale,
            nonce,
        }
    }
}
