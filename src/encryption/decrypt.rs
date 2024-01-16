use chacha20poly1305::ChaCha20Poly1305;

use crate::models::students::student_out::StudentOut;
use crate::models::exams::exam_out::ExamOut;

pub trait DecryptStudent {
    fn decrypt(&self, cipher: &ChaCha20Poly1305) -> StudentOut;
}

pub trait DecryptExam {
    fn decrypt(&self, cipher: &ChaCha20Poly1305) -> ExamOut;
}
