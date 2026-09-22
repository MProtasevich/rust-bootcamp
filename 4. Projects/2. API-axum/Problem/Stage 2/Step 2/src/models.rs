use serde::{Deserialize, Serialize};
use sqlx::types::time::PrimitiveDateTime;
use sqlx::FromRow;
use thiserror::Error;

#[derive(Serialize, Deserialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, FromRow)]
pub struct QuestionDetail {
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: PrimitiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

// ----------

#[derive(Serialize, Deserialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, FromRow)]
pub struct AnswerDetail {
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub answer_uuid: String,
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub question_uuid: String,
    pub content: String,
    pub created_at: PrimitiveDateTime,
}

#[derive(Serialize, Deserialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

// ----------

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occurred")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

// source: https://www.postgresql.org/docs/current/errcodes-appendix.html
pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}