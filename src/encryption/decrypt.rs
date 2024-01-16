use chacha20poly1305::ChaCha20Poly1305;

use crate::models::students::student_out::StudentOut;

pub trait Decrypt {
    fn decrypt(self, cipher: &ChaCha20Poly1305) -> StudentOut;
}
