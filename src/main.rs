use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{routing::get, routing::post, Router};
use consts::API_VERSION;
use databases::database::IDatabase;
use databases::mongo_database::MongoDatabase;
use dotenv::dotenv;
use endpoints::exam::{delete_exam, get_exam, get_exams, post_exam, put_exam};
use endpoints::student::{delete_student, get_student, get_students, post_student, put_student};
use endpoints::welcome;
use std::env;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tower_http::trace::TraceLayer;

mod consts;
mod custom;
mod databases;
mod endpoints;
mod helper;
mod models;

#[derive(Clone)]
pub struct AppState {
    database: Arc<dyn IDatabase + Sync + Send>,
}

#[tokio::main]
async fn main() -> custom::Result<()> {
    // Init logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    // Get secrets
    dotenv().ok();

    let mongo_host = env::var("MONGODB_HOST").unwrap();
    let mongo_database_name = env::var("MONGODB_DATABASE_NAME").unwrap();
    let mongo_user = env::var("MONGODB_USER").unwrap();
    let mongo_password = env::var("MONGODB_PASSWORD").unwrap();

    // Create DB
    let mongo = MongoDatabase::new(
        &mongo_user,
        &mongo_password,
        &mongo_host,
        &mongo_database_name,
    )
    .await?;

    // Create API
    let state = AppState {
        database: Arc::new(mongo),
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:9000".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
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
            .route("/exams", get(get_exams))
            .route("/exams/", post(post_exam))
            .route(
                "/exams/:id",
                get(get_exam).put(put_exam).delete(delete_exam),
            )
            .with_state(state)
            .layer(cors)
            .layer(TraceLayer::new_for_http()),
    );

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
