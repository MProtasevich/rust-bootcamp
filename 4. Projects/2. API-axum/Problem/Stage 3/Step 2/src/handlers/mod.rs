use axum::{extract::State, response::IntoResponse, Json};

use crate::{models::*, AppState};

mod handlers_inner;

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    Json(handlers_inner::create_question(question, &questions_dao).await)
}

pub async fn read_questions(
    State(AppState { questions_dao, .. }): State<AppState>,
) -> impl IntoResponse {
    Json(handlers_inner::read_questions(&questions_dao).await)
}

pub async fn delete_question(
    State(AppState { questions_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(handlers_inner::delete_question(question_uuid, &questions_dao).await)
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    Json(handlers_inner::create_answer(answer, &answers_dao).await)
}

pub async fn read_answers(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(handlers_inner::read_answers(question_uuid, &answers_dao).await)
}

pub async fn delete_answer(
    // TODO: add answers_dao from app state as an argument
    State(AppState { answers_dao, .. }): State<AppState>,
    Json(answer_uuid): Json<AnswerId>,
) -> impl IntoResponse {
    Json(handlers_inner::delete_answer(answer_uuid, &answers_dao).await)
}
