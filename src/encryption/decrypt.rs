use crate::models::students::student_out::StudentOut;

pub trait Decrypt {
    fn decrypt(self, encryption_key: &str) -> StudentOut;
}
