use crate::{
    custom,
    models::{
        exams::{encrypted_exam_in::EncryptedExamIn, encrypted_exam_out::EncryptedExamOut},
        students::{
            encrypted_student_in::EncryptedStudentIn, encrypted_student_out::EncryptedStudentOut,
        },
    },
};
use axum::async_trait;

#[async_trait]
pub trait EncryptedAPIInterface {
    async fn list_students(&self) -> custom::Result<Vec<EncryptedStudentOut>>;
    async fn find_student(&self, id: &str) -> custom::Result<EncryptedStudentOut>;
    async fn insert_student(
        &self,
        student: EncryptedStudentIn,
    ) -> custom::Result<EncryptedStudentOut>;
    async fn replace_student(
        &self,
        id: &str,
        student: EncryptedStudentIn,
    ) -> custom::Result<EncryptedStudentOut>;
    async fn delete_student(&self, id: &str) -> custom::Result<()>;

    async fn list_exams(&self) -> custom::Result<Vec<EncryptedExamOut>>;
    async fn find_exam(&self, id: &str) -> custom::Result<EncryptedExamOut>;
    async fn insert_exam(&self, exam: EncryptedExamIn) -> custom::Result<EncryptedExamOut>;
    async fn replace_exam(
        &self,
        id: &str,
        exam: EncryptedExamIn,
    ) -> custom::Result<EncryptedExamOut>;
    async fn delete_exam(&self, id: &str) -> custom::Result<()>;
}
