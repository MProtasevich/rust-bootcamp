use axum::{response::IntoResponse, Json};
use di_axum::Inject;

use crate::persistance::answers_dao::AnswersDao;
use crate::persistance::questions_dao::QuestionsDao;
use crate::models::*;

mod handlers_inner;

// ---- CRUD for Questions ----

pub async fn create_question(
    // Example of how to add state to a route. Note that we are using ".." to ignore the other fields in AppState.
    Inject(questions_dao): Inject<dyn QuestionsDao>,
    Json(question): Json<Question>,
) -> impl IntoResponse {
    Json(handlers_inner::create_question(question, questions_dao.as_ref()).await)
}

pub async fn read_questions(
    Inject(questions_dao): Inject<dyn QuestionsDao>,
) -> impl IntoResponse {
    Json(handlers_inner::read_questions(questions_dao.as_ref()).await)
}

pub async fn delete_question(
    Inject(questions_dao): Inject<dyn QuestionsDao>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(handlers_inner::delete_question(question_uuid, questions_dao.as_ref()).await)
}

// ---- CRUD for Answers ----

pub async fn create_answer(
    // Example of how to add state to a route
    Inject(answers_dao): Inject<dyn AnswersDao>,
    Json(answer): Json<Answer>,
) -> impl IntoResponse {
    Json(handlers_inner::create_answer(answer, answers_dao.as_ref()).await)
}

pub async fn read_answers(
    Inject(answers_dao): Inject<dyn AnswersDao>,
    Json(question_uuid): Json<QuestionId>,
) -> impl IntoResponse {
    Json(handlers_inner::read_answers(question_uuid, answers_dao.as_ref()).await)
}

pub async fn delete_answer(
    Inject(answers_dao): Inject<dyn AnswersDao>,
    Json(answer_uuid): Json<AnswerId>,
) -> impl IntoResponse {
    Json(handlers_inner::delete_answer(answer_uuid, answers_dao.as_ref()).await)
}
