use chacha20poly1305::ChaCha20Poly1305;

use crate::models::students::encrypted_student_in::EncryptedStudentIn;

pub trait Encrypt {
    fn encrypt(self, encryption_key: &ChaCha20Poly1305) -> EncryptedStudentIn;
}
