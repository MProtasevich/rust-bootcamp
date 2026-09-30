use std::sync::Arc;
use async_trait::async_trait;
use di::injectable;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};
use sqlx::types::Uuid;

#[async_trait]
pub trait AnswersDao: Send + Sync {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: Uuid) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: Uuid) -> Result<Vec<AnswerDetail>, DBError>;
}

#[injectable(AnswersDao)]
pub struct AnswersDaoImpl {
    db: Arc<PgPool>,
}

impl AnswersDaoImpl {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        sqlx::query_as::<_, AnswerDetail>("INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *")
            .bind(answer.question_uuid)
            .bind(answer.content)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(error)
                    if error.code().is_some_and(|code| code == postgres_error_codes::FOREIGN_KEY_VIOLATION) =>
                        DBError::InvalidUUID(error.message().to_string()),
                _ => DBError::Other(Box::new(e))
            })
    }

    async fn delete_answer(&self, answer_uuid: Uuid) -> Result<(), DBError> {
        sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(answer_uuid)
            .execute(self.db.as_ref())
            .await
            .map(|_| ())
            .map_err(|e| DBError::Other(Box::new(e)))
    }

    async fn get_answers(&self, question_uuid: Uuid) -> Result<Vec<AnswerDetail>, DBError> {
        sqlx::query_as::<_, AnswerDetail>("SELECT * FROM answers WHERE question_uuid = $1")
            .bind(question_uuid)
            .fetch_all(self.db.as_ref())
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
    }
}
