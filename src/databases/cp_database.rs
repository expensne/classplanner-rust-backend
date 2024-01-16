use crate::{
    custom,
    models::{
        exams::exam::{Exam, ExamResponse},
        students::{student_in::StudentIn, student_out::StudentOut},
    },
};
use axum::async_trait;

#[async_trait]
pub trait CPDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentOut>>;
    async fn find_student(&self, id: &str) -> custom::Result<StudentOut>;
    async fn insert_student(&self, student: StudentIn) -> custom::Result<StudentOut>;
    async fn replace_student(&self, id: &str, student: StudentIn) -> custom::Result<StudentOut>;
    async fn delete_student(&self, id: &str) -> custom::Result<()>;

    async fn list_exams(&self) -> custom::Result<Vec<ExamResponse>>;
    async fn find_exam(&self, id: &str) -> custom::Result<ExamResponse>;
    async fn insert_exam(&self, exam: Exam) -> custom::Result<ExamResponse>;
    async fn replace_exam(&self, id: &str, exam: Exam) -> custom::Result<ExamResponse>;
    async fn delete_exam(&self, id: &str) -> custom::Result<()>;
}
