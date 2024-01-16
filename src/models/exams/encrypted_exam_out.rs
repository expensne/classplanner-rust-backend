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
    pub maxPoints: f64,
    pub gradingScale: GradingScale,
    pub nonce: String,
}

impl EncryptedExamOut {
    fn try_decrypt(
        &self,
        cipher: &ChaCha20Poly1305,
    ) -> Result<ExamOut, Box<dyn std::error::Error>> {
        let name_decoded = hex::decode(self.name.to_owned())?;

        let nonce_decoded = hex::decode(self.nonce.to_owned())?;
        let nonce = Nonce::from_slice(&nonce_decoded);

        let name_decrypted = cipher
            .decrypt(&nonce, name_decoded.as_ref())
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;

        let name = String::from_utf8(name_decrypted)?;

        Ok(ExamOut {
            id: self.id.to_owned(),
            name,
            maxPoints: self.maxPoints,
            gradingScale: self.gradingScale.to_owned(),
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
                    maxPoints: self.maxPoints,
                    gradingScale: self.gradingScale.to_owned(),
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
        let max_points = doc.get_f64("maxPoints").unwrap();
        let grading_scale = doc.get_document("gradingScale").unwrap();

        // TODO: This is a hack to get around the fact that the nonce is not stored in the database for old data
        // Remove this once all data has been encrypted
        let nonce = match doc.get_str("nonce") {
            Ok(nonce) => nonce.to_string(),
            Err(_) => "missing".to_string(),
        };

        Self {
            id,
            name,
            maxPoints: max_points,
            gradingScale: GradingScale {
                A: grading_scale.get_f64("A").unwrap(),
                B: grading_scale.get_f64("B").unwrap(),
                C: grading_scale.get_f64("C").unwrap(),
                D: grading_scale.get_f64("D").unwrap(),
                E: grading_scale.get_f64("E").unwrap(),
                F: grading_scale.get_f64("F").unwrap(),
            },
            nonce,
        }
    }
}
