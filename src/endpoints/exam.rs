use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};

use crate::{consts::API_VERSION, helper::get_error_response, models::exam::Exam, AppState};

pub async fn get_exams(State(state): State<AppState>) -> impl IntoResponse {
    let exams = state.database.list_exams().await.unwrap();

    (StatusCode::OK, Json(exams))
}

pub async fn get_exam(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let exam = state.database.find_exam(&id).await;

    match exam {
        Ok(exam) => (StatusCode::OK, Json(exam)).into_response(),
        Err(err) => {
            let status = StatusCode::NOT_FOUND;
            let message = err.to_string();

            let error_response =
                get_error_response(&status, &format!("{}/exams/{}", API_VERSION, id), &message);

            (status, error_response).into_response()
        }
    }
}

pub async fn post_exam(
    State(state): State<AppState>,
    Json(payload): Json<Exam>,
) -> impl IntoResponse {
    let exam = state
        .database
        .insert_exam(payload.to_owned())
        .await
        .unwrap();

    (StatusCode::CREATED, Json(exam))
}

pub async fn put_exam(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<Exam>,
) -> Response {
    let exam = state.database.replace_exam(&id, payload.to_owned()).await;

    match exam {
        Ok(exam) => (StatusCode::OK, Json(exam)).into_response(),
        Err(err) => {
            let status = StatusCode::NOT_FOUND;
            let message = err.to_string();

            let error_response =
                get_error_response(&status, &format!("{}/exams/{}", API_VERSION, id), &message);

            (status, error_response).into_response()
        }
    }
}

pub async fn delete_exam(State(state): State<AppState>, Path(id): Path<String>) -> Response {
    let result = state.database.delete_exam(&id).await;

    match result {
        Ok(_) => (StatusCode::OK).into_response(),
        Err(err) => {
            let status = StatusCode::NOT_FOUND;
            let message = err.to_string();

            let error_response =
                get_error_response(&status, &format!("{}/exams/{}", API_VERSION, id), &message);

            (status, error_response).into_response()
        }
    }
}
