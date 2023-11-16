use core::panic;

use axum::async_trait;
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::results::DeleteResult;
use mongodb::{Client, Collection, Cursor, Database};

use crate::custom;
use crate::helper::parse_id;
use crate::models::exam::{Exam, ExamResponse};
use crate::models::student::{Student, StudentResponse};

use super::database::IDatabase;

#[derive(Clone)]
pub struct MongoDatabase {
    database: Database,
    coll_name_students: &'static str,
    coll_name_exams: &'static str,
}

impl MongoDatabase {
    pub async fn new(
        user: &str,
        password: &str,
        host: &str,
        database_name: &str,
    ) -> custom::Result<Self> {
        let uri = format!("mongodb+srv://{user}:{password}@{host}/?retryWrites=true&w=majority");

        let client = Client::with_uri_str(uri).await?;
        let database = client.database(database_name);

        Ok(Self {
            database,
            coll_name_students: "students",
            coll_name_exams: "exams",
        })
    }

    async fn list(&self, collection_name: &str) -> custom::Result<Vec<Document>> {
        let mut cursor: Cursor<Document> = self
            .database
            .collection(collection_name)
            .find(None, None)
            .await?;

        let mut docs: Vec<Document> = Vec::new();

        while let Some(doc) = cursor.next().await {
            docs.push(doc?);
        }

        Ok(docs)
    }

    async fn find(&self, collection_name: &str, id: &str) -> custom::Result<Option<Document>> {
        let id_object = parse_id(id)?;

        let doc: Option<Document> = self
            .database
            .collection(collection_name)
            .find_one(doc! {"_id": id_object}, None)
            .await?;

        Ok(doc)
    }
}

#[async_trait]
impl IDatabase for MongoDatabase {
    async fn list_students(&self) -> custom::Result<Vec<StudentResponse>> {
        let docs: Vec<Document> = self.list(self.coll_name_students).await.unwrap();
        let students: Vec<StudentResponse> = docs.into_iter().map(StudentResponse::from).collect();

        Ok(students)
    }

    async fn find_student(&self, id: &str) -> custom::Result<StudentResponse> {
        let doc = self.find(self.coll_name_students, id).await?;

        match doc {
            Some(doc) => Ok(StudentResponse::from(doc)),
            None => Err(From::from("Student not found")),
        }
    }

    async fn insert_student(&self, student: Student) -> custom::Result<StudentResponse> {
        let result = self
            .database
            .collection(self.coll_name_students)
            .insert_one(student.to_owned(), None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        let mut response = StudentResponse::from(student);
        response.id = id.to_string();
        Ok(response)
    }

    async fn replace_student(&self, id: &str, student: Student) -> custom::Result<StudentResponse> {
        let id_object = parse_id(id)?;

        let result = self
            .database
            .collection(self.coll_name_students)
            .replace_one(doc! {"_id": id_object}, student.clone(), None)
            .await?;

        match result.modified_count {
            1 => {
                let mut response = StudentResponse::from(student);
                response.id = id.to_string();
                Ok(response)
            }
            0 => Err(From::from("Student not found")),
            _ => panic!(),
        }
    }

    async fn delete_student(&self, id: &str) -> custom::Result<()> {
        let id_object = parse_id(id)?;

        let result: DeleteResult = self
            .database
            .collection::<Collection<StudentResponse>>(self.coll_name_students)
            .delete_one(doc! {"_id": id_object}, None)
            .await?;

        match result.deleted_count {
            1 => Ok(()),
            0 => Err(From::from("Student not found")),
            _ => panic!(),
        }
    }

    async fn list_exams(&self) -> custom::Result<Vec<ExamResponse>> {
        let docs: Vec<Document> = self.list(self.coll_name_exams).await.unwrap();

        let students: Vec<ExamResponse> = docs.into_iter().map(ExamResponse::from).collect();

        Ok(students)
    }

    async fn find_exam(&self, id: &str) -> custom::Result<ExamResponse> {
        let doc = self.find(self.coll_name_exams, id).await?;

        match doc {
            Some(doc) => Ok(ExamResponse::from(doc)),
            None => Err(From::from("Exam not found")),
        }
    }

    async fn insert_exam(&self, exam: Exam) -> custom::Result<ExamResponse> {
        let result = self
            .database
            .collection(self.coll_name_exams)
            .insert_one(exam.to_owned(), None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        let mut response = ExamResponse::from(exam);
        response.id = id.to_string();
        Ok(response)
    }

    async fn replace_exam(&self, id: &str, exam: Exam) -> custom::Result<ExamResponse> {
        let id_object = parse_id(id)?;

        let result = self
            .database
            .collection(self.coll_name_exams)
            .replace_one(doc! {"_id": id_object}, exam.clone(), None)
            .await?;

        match result.modified_count {
            1 => {
                let mut response = ExamResponse::from(exam);
                response.id = id.to_string();
                Ok(response)
            }
            0 => Err(From::from("Exam not found")),
            _ => panic!(),
        }
    }

    async fn delete_exam(&self, id: &str) -> custom::Result<()> {
        let id_object = parse_id(id)?;

        let result: DeleteResult = self
            .database
            .collection::<Collection<ExamResponse>>(self.coll_name_exams)
            .delete_one(doc! {"_id": id_object}, None)
            .await?;

        match result.deleted_count {
            1 => Ok(()),
            0 => Err(From::from("Exam not found")),
            _ => panic!(),
        }
    }
}
