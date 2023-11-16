use core::panic;

use futures::StreamExt;
use mongodb::bson::oid::ObjectId;
use mongodb::bson::{doc, Document};
use mongodb::error::Result;
use mongodb::results::DeleteResult;
use mongodb::{Client, Collection, Cursor, Database};
use serde::{Deserialize, Serialize};
use tracing::event;

#[derive(Deserialize, Serialize, Clone)]
pub struct InsertStudent {
    pub firstname: String,
    pub lastname: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Student {
    pub id: String,
    pub firstname: String,
    pub lastname: String,
}

impl Default for Student {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            firstname: "".to_string(),
            lastname: "".to_string(),
        }
    }
}

#[derive(Clone)]
pub struct Mongo {
    database: Database,
    coll_name_students: &'static str,
    coll_name_exams: &'static str,
}

impl Mongo {
    pub async fn new(user: &str, password: &str, host: &str, database_name: &str) -> Result<Self> {
        let uri = format!("mongodb+srv://{user}:{password}@{host}/?retryWrites=true&w=majority");

        let client = Client::with_uri_str(uri).await?;
        let database = client.database(database_name);

        Ok(Self {
            database,
            coll_name_students: "students",
            coll_name_exams: "exams",
        })
    }

    async fn find_one(&self, coll_name: &str, filter: Document) -> Option<Document> {
        self.database
            .collection(coll_name)
            .find_one(filter, None)
            .await
            .unwrap()
    }

    async fn find_all(&self, coll_name: &str) -> Result<Vec<Document>> {
        let mut cursor: Cursor<Document> =
            self.database.collection(coll_name).find(None, None).await?;

        let mut docs = Vec::new();

        while let Some(doc) = cursor.next().await {
            docs.push(doc?);
        }

        Ok(docs)
    }

    pub async fn find_student(&self, id: &str) -> Result<Student> {
        let doc = self
            .find_one(
                self.coll_name_students,
                doc! {"_id": id.parse::<ObjectId>().unwrap()},
            )
            .await;

        match doc {
            Some(doc) => {
                let id = doc.get_object_id("_id").unwrap().to_hex();
                let firstname = doc.get_str("firstname").unwrap().to_string();
                let lastname = doc.get_str("lastname").unwrap().to_string();

                Ok(Student {
                    id,
                    firstname,
                    lastname,
                })
            }
            None => Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Student not found",
            ))),
        }
    }

    pub async fn find_students(&self) -> Result<Vec<Student>> {
        let docs = self.find_all(self.coll_name_students).await?;

        let students: Vec<Student> = docs
            .into_iter()
            .map(|doc| {
                let id = doc.get_object_id("_id").unwrap().to_hex();
                let firstname = doc.get_str("firstname").unwrap().to_string();
                let lastname = doc.get_str("lastname").unwrap().to_string();

                Student {
                    id,
                    firstname,
                    lastname,
                }
            })
            .collect();

        Ok(students)
    }

    pub async fn insert_student(&self, student: InsertStudent) -> Result<Student> {
        let result = self
            .database
            .collection(self.coll_name_students)
            .insert_one(student.to_owned(), None)
            .await?;

        let id = result.inserted_id.as_object_id().unwrap().to_hex();

        Ok(Student {
            id,
            firstname: student.firstname,
            lastname: student.lastname,
        })
    }

    pub async fn replace_student(&self, student: Student) -> Result<Student> {
        let result = self
            .database
            .collection(self.coll_name_students)
            .replace_one(
                doc! {"_id": student.id.parse::<ObjectId>().unwrap()},
                doc! {
                    "firstname": student.firstname.to_owned(),
                    "lastname": student.lastname.to_owned()
                },
                None,
            )
            .await?;

        match result.modified_count {
            1 => Ok(student),
            0 => Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Student not found",
            ))),
            _ => panic!(),
        }
    }

    pub async fn delete_student(&self, id: &str) -> Result<()> {
        let result: DeleteResult = self
            .database
            .collection::<Collection<Student>>(self.coll_name_students)
            .delete_one(doc! {"_id": id.parse::<ObjectId>().unwrap()}, None)
            .await?;

            match result.deleted_count {
            1 => Ok(()),
            0 => Err(mongodb::error::Error::from(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Student not found",
            ))),
            _ => panic!(),
        }
    }
}
