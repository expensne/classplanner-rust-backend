use axum::http::header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE};
use axum::http::{HeaderValue, Method};
use axum::{routing::get, routing::post, Router};
use chacha20poly1305::{aead::KeyInit, ChaCha20Poly1305};
use consts::API_VERSION;
use databases::ciphered_database::CipheredDatabase;
use databases::interfaces::api_interface::APIInterface;
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
mod encryption;
mod endpoints;
mod helper;
mod models;

#[derive(Clone)]
pub struct AppState {
    database: Arc<dyn APIInterface + Sync + Send>,
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
    let encryption_key = env::var("ENCRYPTION_KEY").unwrap();

    // Create DB
    let mongo = MongoDatabase::new(
        &mongo_user,
        &mongo_password,
        &mongo_host,
        &mongo_database_name,
    )
    .await?;

    let cipher = ChaCha20Poly1305::new_from_slice(encryption_key.as_bytes()).unwrap();
    let ciphered_database = CipheredDatabase::new(Box::new(mongo), cipher).unwrap();

    // Create API
    let state = AppState {
        database: Arc::new(ciphered_database),
    };

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::PUT, Method::POST, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE, ACCEPT]);

    let api_routes = Router::new()
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
        .with_state(state);

    let app = Router::new()
        .nest(API_VERSION, api_routes)
        .layer(cors)
        .layer(TraceLayer::new_for_http());

    axum::Server::bind(&"0.0.0.0:5000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
