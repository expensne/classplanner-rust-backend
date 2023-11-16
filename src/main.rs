use axum::extract::{Path, State};
use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method, StatusCode};
use axum::response::IntoResponse;
use axum::Json;
use axum::{routing::delete, routing::get, routing::post, routing::put, Router};
use dotenv::dotenv;
use mongodb::bson::{self, doc, document};
use serde::{Deserialize, Serialize};
use std::env;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::event;

use crate::db::Student;

mod db;

const API_VERSION: &str = "/v1";

#[derive(Clone)]
struct AppState {
    db: db::Mongo,
}

#[tokio::main]
async fn main() -> mongodb::error::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    dotenv().ok();

    let mongo_host = env::var("MONGODB_HOST").unwrap();
    let mongo_database_name = env::var("MONGODB_DATABASE_NAME").unwrap();
    let mongo_user = env::var("MONGODB_USER").unwrap();
    let mongo_password = env::var("MONGODB_PASSWORD").unwrap();

    let mongo = db::Mongo::new(
        &mongo_user,
        &mongo_password,
        &mongo_host,
        &mongo_database_name,
    )
    .await?;

    let state = AppState { db: mongo };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5000".parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::PUT,
            Method::POST,
            //Method::PATCH,
            Method::DELETE,
        ])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let app = Router::new().nest(
        API_VERSION,
        Router::new()
            .route("/", get(welcome))
            .route("/students", get(get_students))
            .route("/students/", post(post_student))
            .route(
                "/students/:id",
                get(get_student).put(put_student).delete(delete_student),
            )
            .with_state(state)
            .layer(cors)
            .layer(TraceLayer::new_for_http()),
    );

    axum::Server::bind(&"127.0.0.1:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

async fn welcome() -> String {
    format!("Welcome to Classplanner API {API_VERSION}")
}

async fn get_student(State(state): State<AppState>, Path(id): Path<String>) -> impl IntoResponse {
    let student = state.db.find_student(&id).await;
    match student {
        Ok(student) => (StatusCode::OK, Json(student)),
        Err(err) => {
            event!(tracing::Level::ERROR, "Error getting student: {}", err);
            (StatusCode::NOT_FOUND, Json(Student::default()))
        }
    }
}

async fn get_students(State(state): State<AppState>) -> impl IntoResponse {
    let students = state.db.find_students().await.unwrap();
    (StatusCode::OK, Json(students))
}

async fn post_student(
    State(state): State<AppState>,
    Json(payload): Json<db::InsertStudent>,
) -> impl IntoResponse {
    let student = state.db.insert_student(payload.to_owned()).await.unwrap();
    (StatusCode::CREATED, Json(student))
}

async fn put_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<db::Student>,
) -> impl IntoResponse {
    if payload.id != id {
        return (StatusCode::BAD_REQUEST, Json(payload));
    }

    let student = state.db.replace_student(payload.to_owned()).await;

    match student {
        Ok(student) => (StatusCode::OK, Json(student)),
        Err(err) => {
            event!(tracing::Level::ERROR, "Error updating student: {}", err);
            (StatusCode::NOT_FOUND, Json(payload))
        }
    }
}

async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let result = state.db.delete_student(&id).await;

    match result {
        Ok(_) => (StatusCode::OK, Json(Student::default())),
        Err(err) => {
            event!(
                tracing::Level::ERROR,
                "Error deleting student with id: {}",
                err
            );
            (StatusCode::NO_CONTENT, Json(Student::default()))
        }
    }
}
