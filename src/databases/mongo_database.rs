use super::interfaces::ciphered_api_interface::CipheredAPIInterface;
use crate::custom;
use crate::helper::parse_id;
use crate::models::exams::encrypted_exam_in::EncryptedExamIn;
use crate::models::exams::encrypted_exam_out::EncryptedExamOut;
use crate::models::students::encrypted_student_in::EncryptedStudentIn;
use crate::models::students::encrypted_student_out::EncryptedStudentOut;
use axum::async_trait;
use core::panic;
use futures::StreamExt;
use mongodb::bson::{doc, Document};
use mongodb::results::DeleteResult;
use mongodb::{Client, Collection, Cursor, Database};

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
impl CipheredAPIInterface for MongoDatabase {
    async fn list_students(&self) -> custom::Result<Vec<EncryptedStudentOut>> {
        tracing::info!("Listing students");
        let docs: Vec<Document> = self.list(self.coll_name_students).await.unwrap();
        let students: Vec<EncryptedStudentOut> =
            docs.into_iter().map(EncryptedStudentOut::from).collect();

        Ok(students)
    }

    async fn find_student(&self, id: &str) -> custom::Result<EncryptedStudentOut> {
        tracing::info!("Finding student with id {}", id);
        let doc = self.find(self.coll_name_students, id).await?;

        match doc {
            Some(doc) => Ok(EncryptedStudentOut::from(doc)),
            None => Err(From::from("Student not found")),
        }
    }

    async fn insert_student(
        &self,
        student: EncryptedStudentIn,
    ) -> custom::Result<EncryptedStudentOut> {
        tracing::info!("Inserting student {:?}", student);
        let result = self
            .database
            .collection(self.coll_name_students)
            .insert_one(student.to_owned(), None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        let mut response = EncryptedStudentOut::from(student);
        response.id = id.to_string();
        Ok(response)
    }

    async fn replace_student(
        &self,
        id: &str,
        student: EncryptedStudentIn,
    ) -> custom::Result<EncryptedStudentOut> {
        tracing::info!("Replacing student with id {} with {:?}", id, student);
        let id_object = parse_id(id)?;

        let result = self
            .database
            .collection(self.coll_name_students)
            .replace_one(doc! {"_id": id_object}, student.clone(), None)
            .await?;

        match result.modified_count {
            1 => {
                let mut response = EncryptedStudentOut::from(student);
                response.id = id.to_string();
                Ok(response)
            }
            0 => Err(From::from("Student not found")),
            _ => panic!(),
        }
    }

    async fn delete_student(&self, id: &str) -> custom::Result<()> {
        tracing::info!("Deleting student with id {}", id);
        let id_object = parse_id(id)?;

        let result: DeleteResult = self
            .database
            .collection::<Collection<EncryptedStudentOut>>(self.coll_name_students)
            .delete_one(doc! {"_id": id_object}, None)
            .await?;

        match result.deleted_count {
            1 => Ok(()),
            0 => Err(From::from("Student not found")),
            _ => panic!(),
        }
    }

    async fn list_exams(&self) -> custom::Result<Vec<EncryptedExamOut>> {
        let docs: Vec<Document> = self.list(self.coll_name_exams).await.unwrap();
        let students: Vec<EncryptedExamOut> =
            docs.into_iter().map(EncryptedExamOut::from).collect();

        Ok(students)
    }

    async fn find_exam(&self, id: &str) -> custom::Result<EncryptedExamOut> {
        let doc = self.find(self.coll_name_exams, id).await?;

        match doc {
            Some(doc) => Ok(EncryptedExamOut::from(doc)),
            None => Err(From::from("Exam not found")),
        }
    }

    async fn insert_exam(&self, exam: EncryptedExamIn) -> custom::Result<EncryptedExamOut> {
        let result = self
            .database
            .collection(self.coll_name_exams)
            .insert_one(exam.to_owned(), None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        let mut response = EncryptedExamOut::from(exam);
        response.id = id.to_string();
        Ok(response)
    }

    async fn replace_exam(
        &self,
        id: &str,
        exam: EncryptedExamIn,
    ) -> custom::Result<EncryptedExamOut> {
        let id_object = parse_id(id)?;

        let result = self
            .database
            .collection(self.coll_name_exams)
            .replace_one(doc! {"_id": id_object}, exam.clone(), None)
            .await?;

        match result.modified_count {
            1 => {
                let mut response = EncryptedExamOut::from(exam);
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
            .collection::<Collection<EncryptedExamOut>>(self.coll_name_exams)
            .delete_one(doc! {"_id": id_object}, None)
            .await?;

        match result.deleted_count {
            1 => Ok(()),
            0 => Err(From::from("Exam not found")),
            _ => panic!(),
        }
    }
}
