use crate::models::*;
use axum::{response::IntoResponse, Json};
use chrono::Utc;
use uuid::Uuid;
// ---- CRUD for Questions ----

pub async fn create_question(Json(question): Json<Question>) -> impl IntoResponse {
    Json(QuestionDetail {
        question_uuid: Uuid::new_v4().to_string(),
        title: question.title,
        description: question.description,
        created_at: Utc::now().to_rfc3339(),
    })
}

pub async fn read_questions() -> impl IntoResponse {
    Json(vec![QuestionDetail {
        question_uuid: "question_uuid".to_owned(),
        title: "title".to_owned(),
        description: "description".to_owned(),
        created_at: "created_at".to_owned(),
    }])
}

pub async fn delete_question(Json(question_uuid): Json<QuestionId>) {
    // ...
}

// ---- CRUD for Answers ----

// TODO: Create a POST route to /answer which accepts an `Answer` and returns `AnswerDetail` as JSON.
//       The handler function should be called `create_answer`.
//
//       hint: this function should look very similar to the create_question function above

// TODO: Create a GET route to /answers which accepts an `QuestionId` and returns a vector of `AnswerDetail` as JSON.
//       The handler function should be called `read_answers`.
//
//       hint: this function should look very similar to the read_questions function above

// TODO: Create a DELETE route to /answer which accepts an `AnswerId` and does not return anything.
//       The handler function should be called `delete_answer`.
//
//       hint: this function should look very similar to the delete_question function above
