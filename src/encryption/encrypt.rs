use chacha20poly1305::ChaCha20Poly1305;

use crate::models::{
    exams::encrypted_exam_in::EncryptedExamIn, students::encrypted_student_in::EncryptedStudentIn,
};

pub trait EncryptStudent {
    fn encrypt(&self, cipher: &ChaCha20Poly1305) -> EncryptedStudentIn;
}

pub trait EncryptExam {
    fn encrypt(&self, cipher: &ChaCha20Poly1305) -> EncryptedExamIn;
}
