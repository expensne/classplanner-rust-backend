use crate::models::students::encrypted_student_in::EncryptedStudentIn;

pub trait Encrypt {
    fn encrypt(self, encryption_key: &str) -> EncryptedStudentIn;
}
