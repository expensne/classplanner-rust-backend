use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{consts::API_VERSION, helper::get_error_response, models::student::Student, AppState};



pub async fn get_student(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let student = state.database.find_student(&id).await;

    match student {
        Ok(student) => (StatusCode::OK, Json(student)).into_response(),
        Err(err) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let message = err.to_string();

            let error_response = get_error_response(
                &status,
                &format!("{}/students/{}", API_VERSION, id),
                &message,
            );

            (status, error_response).into_response()
        }
    }
}

pub async fn get_students(State(state): State<AppState>) -> impl IntoResponse {
    let students = state.database.list_students().await.unwrap();

    (StatusCode::OK, Json(students))
}

pub async fn post_student(
    State(state): State<AppState>,
    Json(payload): Json<Student>,
) -> impl IntoResponse {
    let student = state
        .database
        .insert_student(payload.to_owned())
        .await
        .unwrap();

    (StatusCode::CREATED, Json(student))
}

pub async fn put_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<Student>,
) -> Response {
    let student = state
        .database
        .replace_student(&id, payload.to_owned())
        .await;

    match student {
        Ok(student) => (StatusCode::OK, Json(student)).into_response(),
        Err(err) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let message = err.to_string();

            let error_response = get_error_response(
                &status,
                &format!("{}/students/{}", API_VERSION, id),
                &message,
            );

            (status, error_response).into_response()
        }
    }
}

pub async fn delete_student(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let result = state.database.delete_student(&id).await;

    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(err) => {
            let status = StatusCode::INTERNAL_SERVER_ERROR;
            let message = err.to_string();

            let error_response = get_error_response(
                &status,
                &format!("{}/students/{}", API_VERSION, id),
                &message,
            );

            (status, error_response).into_response()
        }
    }
}
