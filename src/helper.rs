use axum::{http::StatusCode, Json};
use mongodb::bson::{doc, oid::ObjectId, Document};

use crate::custom;

pub fn get_error_response(status: &StatusCode, path: &str, message: &str) -> Json<Document> {
    Json(doc! {
        "error": status.canonical_reason(),
        "status": status.as_str(),
        "message": message,
        "path": path,
    })
}

pub fn parse_id(id: &str) -> custom::Result<ObjectId> {
    let id_object = id.parse::<ObjectId>();

    if id_object.is_err() {
        return Err(From::from("Invalid id format"));
    }

    Ok(id_object.unwrap())
}
