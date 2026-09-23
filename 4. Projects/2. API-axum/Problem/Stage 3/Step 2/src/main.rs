#[macro_use]
extern crate log;

extern crate pretty_env_logger;

use std::sync::Arc;

use axum::{Router, routing::{delete, get, post}};
use di::{ServiceCollection, ServiceProvider, singleton, singleton_as_self};
use di_axum::prelude::*;
use dotenvy::dotenv;
use persistance::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use tokio::runtime::Runtime;

mod handlers;
mod models;
mod persistance;

use handlers::*;

macro_rules! enclose {
    ( ($( $x:ident ),*) $y:expr ) => {
        {
            $(let $x = Arc::clone(&$x);)*
            $y
        }
    };
}

fn get_di_container() -> ServiceProvider {
    let runtime = Arc::new(Runtime::new().unwrap());
    ServiceCollection::new()
        .add(singleton_as_self::<Pool<Postgres>>().from(enclose! { (runtime) move |_| Arc::new(get_pg_pool(Arc::clone(&runtime))) } ))
        .add(singleton::<dyn QuestionsDao, QuestionsDaoImpl>().from(|sp| Arc::new(QuestionsDaoImpl::new(sp.get_required()))))
        .add(singleton::<dyn AnswersDao, AnswersDaoImpl>().from(|sp| Arc::new(AnswersDaoImpl::new(sp.get_required()))))
        .build_provider()
        .expect("Unable to create DI configuration")
}

fn get_pg_pool(runtime: Arc<Runtime>) -> Pool<Postgres> {
    runtime.block_on(PgPoolOptions::new()
        .max_connections(5)
        .connect(&std::env::var("DATABASE_URL").expect("DATABASE_URL must be set.")))
        .expect("Failed to create Postgres connection pool!")
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenv().ok();

    let service_provider = get_di_container();

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        // The with_state method allows us to add state to the state managed by this instance of Axum. Then we can use this state in the handlers.
        .with_provider(service_provider);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
