use crate::{
    custom,
    models::{
        exam::{Exam, ExamResponse},
        student::{Student, StudentResponse},
    },
};
use axum::async_trait;

#[async_trait]
pub trait CPDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentResponse>>;
    async fn find_student(&self, id: &str) -> custom::Result<StudentResponse>;
    async fn insert_student(&self, student: Student) -> custom::Result<StudentResponse>;
    async fn replace_student(&self, id: &str, student: Student) -> custom::Result<StudentResponse>;
    async fn delete_student(&self, id: &str) -> custom::Result<()>;

    async fn list_exams(&self) -> custom::Result<Vec<ExamResponse>>;
    async fn find_exam(&self, id: &str) -> custom::Result<ExamResponse>;
    async fn insert_exam(&self, exam: Exam) -> custom::Result<ExamResponse>;
    async fn replace_exam(&self, id: &str, exam: Exam) -> custom::Result<ExamResponse>;
    async fn delete_exam(&self, id: &str) -> custom::Result<()>;
}
