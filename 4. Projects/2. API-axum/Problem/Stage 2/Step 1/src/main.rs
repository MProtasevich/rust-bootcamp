// TODO: import log, pretty_env_logger, dotenv, and PgPoolOptions

use std::env;
use axum::{
    routing::{delete, get, post},
    Router,
};
use dotenvy::dotenv;
use log::{error, info};
use sqlx::postgres::PgPoolOptions;

mod handlers;
mod models;

use handlers::*;
use crate::models::{Question, QuestionDetail};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let Ok(env) = dotenv().expect("Cannot read .env file");

    // Create a new PgPoolOptions instance with a maximum of 5 connections.
    // Use dotenv to get the database url.
    // Use the `unwrap` or `expect` method instead of handling errors. If an
    // error occurs at this stage the server should be terminated.
    // See examples on GitHub page: https://github.com/launchbadge/sqlx
    let db_url = env::var("DATABASE_URL").expect("No DATABASE_URL defined");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url.as_str()).await.expect("Database connection failed");

    // Using slqx, execute a SQL query that selects all questions from the questions table.
    // Use the `unwrap` or `expect` method to handle errors. This is just some test code to
    // make sure we can connect to the database.
    let recs = sqlx::query_as::<_, QuestionDetail>("SELECT * FROM questions")
        .fetch_all(&pool).await.expect("Unable to load questions");

    info!("********* Question Records *********");
    info!("{:?}", recs);

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
