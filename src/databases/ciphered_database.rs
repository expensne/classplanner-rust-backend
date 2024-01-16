use axum::async_trait;

use crate::{
    custom,
    encryption::decrypt::Decrypt,
    encryption::encrypt::Encrypt,
    models::{
        exams::exam::{Exam, ExamResponse},
        students::{student_in::StudentIn, student_out::StudentOut},
    },
};

use super::{api_interface::APIInterface, encrypted_api_interface::EncryptedAPIInterface};

pub struct CipheredDatabase {
    database: Box<dyn EncryptedAPIInterface + Send + Sync>,
    key: String,
}

impl CipheredDatabase {
    pub fn new(
        database: Box<dyn EncryptedAPIInterface + Send + Sync>,
        key: String,
    ) -> custom::Result<Self> {
        Ok(Self { database, key })
    }
}

#[async_trait]
impl APIInterface for CipheredDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentOut>> {
        let encrypted_students = self.database.list_students().await?;
        let students = encrypted_students
            .into_iter()
            .map(|student| student.decrypt(&self.key))
            .collect();
        
        Ok(students)
    }

    async fn find_student(&self, id: &str) -> custom::Result<StudentOut> {
        let encrypted_student = self.database.find_student(id).await?;
        let student = encrypted_student.decrypt(&self.key);
        Ok(student)
    }

    async fn insert_student(&self, student: StudentIn) -> custom::Result<StudentOut> {
        let encrypted_student = student.encrypt(&self.key);
        let inserted_encrypted_student = self.database.insert_student(encrypted_student).await?;
        let inserted_student = inserted_encrypted_student.decrypt(&self.key);
        Ok(inserted_student)
    }

    async fn replace_student(&self, id: &str, student: StudentIn) -> custom::Result<StudentOut> {
        let encrypted_student = student.encrypt(&self.key);
        let replaced_encrypted_student =
            self.database.replace_student(id, encrypted_student).await?;
        let replaced_student = replaced_encrypted_student.decrypt(&self.key);
        Ok(replaced_student)
    }

    async fn delete_student(&self, id: &str) -> custom::Result<()> {
        self.database.delete_student(id).await
    }

    async fn list_exams(&self) -> custom::Result<Vec<ExamResponse>> {
        self.database.list_exams().await
    }

    async fn find_exam(&self, id: &str) -> custom::Result<ExamResponse> {
        self.database.find_exam(id).await
    }

    async fn insert_exam(&self, exam: Exam) -> custom::Result<ExamResponse> {
        self.database.insert_exam(exam).await
    }

    async fn replace_exam(&self, id: &str, exam: Exam) -> custom::Result<ExamResponse> {
        self.database.replace_exam(id, exam).await
    }

    async fn delete_exam(&self, id: &str) -> custom::Result<()> {
        self.database.delete_exam(id).await
    }
}
