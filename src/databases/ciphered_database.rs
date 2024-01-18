use axum::async_trait;
use chacha20poly1305::ChaCha20Poly1305;

use crate::{
    custom,
    encryption::decrypt::DecryptStudent,
    encryption::{
        decrypt::DecryptExam,
        encrypt::{EncryptExam, EncryptStudent},
    },
    models::{
        exams::{exam_in::ExamIn, exam_out::ExamOut},
        students::{student_in::StudentIn, student_out::StudentOut},
    },
};

use super::{
    interfaces::api_interface::APIInterface,
    interfaces::ciphered_api_interface::CipheredAPIInterface,
};

pub struct CipheredDatabase {
    database: Box<dyn CipheredAPIInterface + Send + Sync>,
    cipher: ChaCha20Poly1305,
}

impl CipheredDatabase {
    pub fn new(
        database: Box<dyn CipheredAPIInterface + Send + Sync>,
        cipher: ChaCha20Poly1305,
    ) -> custom::Result<Self> {
        Ok(Self { database, cipher })
    }
}

#[async_trait]
impl APIInterface for CipheredDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentOut>> {
        tracing::info!("Listing students");
        let encrypted_students = self.database.list_students().await?;
        let students = encrypted_students
            .into_iter()
            .map(|student| student.decrypt(&self.cipher))
            .collect();
        Ok(students)
    }

    async fn find_student(&self, id: &str) -> custom::Result<StudentOut> {
        tracing::info!("Finding student with id {}", id);
        let encrypted_student = self.database.find_student(id).await?;
        let student = encrypted_student.decrypt(&self.cipher);
        Ok(student)
    }

    async fn insert_student(&self, student: StudentIn) -> custom::Result<StudentOut> {
        tracing::info!("Inserting student {:?}", student);
        let encrypted_student = student.encrypt(&self.cipher);
        let inserted_encrypted_student = self.database.insert_student(encrypted_student).await?;
        let inserted_student = inserted_encrypted_student.decrypt(&self.cipher);
        Ok(inserted_student)
    }

    async fn replace_student(&self, id: &str, student: StudentIn) -> custom::Result<StudentOut> {
        tracing::info!("Replacing student with id {} with {:?}", id, student);
        let encrypted_student = student.encrypt(&self.cipher);
        let replaced_encrypted_student =
            self.database.replace_student(id, encrypted_student).await?;
        let replaced_student = replaced_encrypted_student.decrypt(&self.cipher);
        Ok(replaced_student)
    }

    async fn delete_student(&self, id: &str) -> custom::Result<()> {
        tracing::info!("Deleting student with id {}", id);
        self.database.delete_student(id).await
    }

    async fn list_exams(&self) -> custom::Result<Vec<ExamOut>> {
        tracing::info!("Listing exams");
        let encrypted_exams = self.database.list_exams().await?;
        let exams = encrypted_exams
            .into_iter()
            .map(|exam| exam.decrypt(&self.cipher))
            .collect();
        Ok(exams)
    }

    async fn find_exam(&self, id: &str) -> custom::Result<ExamOut> {
        tracing::info!("Finding exam with id {}", id);
        let encrypted_exam = self.database.find_exam(id).await?;
        let exam = encrypted_exam.decrypt(&self.cipher);
        Ok(exam)
    }

    async fn insert_exam(&self, exam: ExamIn) -> custom::Result<ExamOut> {
        tracing::info!("Inserting exam {:?}", exam);
        let encrypted_exam = exam.encrypt(&self.cipher);
        let inserted_encrypted_exam = self.database.insert_exam(encrypted_exam).await?;
        let inserted_exam = inserted_encrypted_exam.decrypt(&self.cipher);
        Ok(inserted_exam)
    }

    async fn replace_exam(&self, id: &str, exam: ExamIn) -> custom::Result<ExamOut> {
        tracing::info!("Replacing exam with id {} with {:?}", id, exam);
        let encrypted_exam = exam.encrypt(&self.cipher);
        let replaced_encrypted_exam = self.database.replace_exam(id, encrypted_exam).await?;
        let replaced_exam = replaced_encrypted_exam.decrypt(&self.cipher);
        Ok(replaced_exam)
    }

    async fn delete_exam(&self, id: &str) -> custom::Result<()> {
        tracing::info!("Deleting exam with id {}", id);
        self.database.delete_exam(id).await
    }
}
