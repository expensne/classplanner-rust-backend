use axum::async_trait;
use mongodb::bson::de;

use crate::{
    custom,
    models::{
        exam::{Exam, ExamResponse},
        student::{Student, StudentResponse},
    },
};

use super::cp_database::CPDatabase;

pub struct EncryptedDatabase {
    database: Box<dyn CPDatabase + Send + Sync>,
    ecryption_key: String,
}

impl EncryptedDatabase {
    pub async fn new(
        database: Box<dyn CPDatabase + Send + Sync>,
        ecryption_key: String,
    ) -> custom::Result<Self> {
        Ok(Self {
            database,
            ecryption_key,
        })
    }

    fn encrypt(&self, text: String) -> String {
        text + "_encrypted"
    }

    fn decrypt(&self, text: String) -> String {
        text.replace("_encrypted", "")
    }
}

#[async_trait]
impl CPDatabase for EncryptedDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentResponse>> {
        let encrypted_students = self.database.list_students().await?;

        let decrypted_students = encrypted_students
            .into_iter()
            .map(|encrypted_student| StudentResponse {
                id: encrypted_student.id,
                firstName: self.decrypt(encrypted_student.firstName),
                lastName: self.decrypt(encrypted_student.lastName),
                scores: encrypted_student.scores,
            })
            .collect();

        Ok(decrypted_students)
    }

    async fn find_student(&self, id: &str) -> custom::Result<StudentResponse> {
        let encrypted_student = self.database.find_student(id).await?;

        Ok(StudentResponse {
            id: encrypted_student.id,
            firstName: self.decrypt(encrypted_student.firstName),
            lastName: self.decrypt(encrypted_student.lastName),
            scores: encrypted_student.scores,
        })
    }

    async fn insert_student(&self, student: Student) -> custom::Result<StudentResponse> {
        let encrypted_student = Student {
            firstName: self.encrypt(student.firstName),
            lastName: self.encrypt(student.lastName),
            scores: student.scores,
        };
        self.database.insert_student(encrypted_student).await
    }

    async fn replace_student(&self, id: &str, student: Student) -> custom::Result<StudentResponse> {
        let encrypted_student = Student {
            firstName: self.encrypt(student.firstName),
            lastName: self.encrypt(student.lastName),
            scores: student.scores,
        };
        self.database.replace_student(id, encrypted_student).await
    }

    async fn delete_student(&self, id: &str) -> custom::Result<()> {
        self.database.delete_student(id).await
    }

    async fn list_exams(&self) -> custom::Result<Vec<ExamResponse>> {
        todo!();
    }

    async fn find_exam(&self, id: &str) -> custom::Result<ExamResponse> {
        todo!();
    }

    async fn insert_exam(&self, exam: Exam) -> custom::Result<ExamResponse> {
        todo!();
    }

    async fn replace_exam(&self, id: &str, exam: Exam) -> custom::Result<ExamResponse> {
        todo!();
    }

    async fn delete_exam(&self, id: &str) -> custom::Result<()> {
        todo!();
    }
}
