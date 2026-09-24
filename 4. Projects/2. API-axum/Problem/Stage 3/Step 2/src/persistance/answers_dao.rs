use std::sync::Arc;
use async_trait::async_trait;
use di::injectable;
use sqlx::PgPool;

use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};
use sqlx::types::Uuid;
use std::str::FromStr;

#[async_trait]
pub trait AnswersDao: Send + Sync {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
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
        let uuid = Uuid::from_str(answer.question_uuid.as_str())
            .map_err(|err| DBError::InvalidUUID(format!("{} cannot be parsed as UUID: {err}", answer.question_uuid)))?;
        sqlx::query_as::<_, AnswerDetail>("INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *")
            .bind(uuid)
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

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::from_str(&answer_uuid)
            .map_err(|err| DBError::InvalidUUID(format!("{answer_uuid} cannot be parsed as UUID: {err}")))?;

        sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(uuid)
            .execute(self.db.as_ref())
            .await
            .map(|_| ())
            .map_err(|e| DBError::Other(Box::new(e)))
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::from_str(&question_uuid)
            .map_err(|err| DBError::InvalidUUID(format!("{question_uuid} cannot be parsed as UUID: {err}")))?;

        sqlx::query_as::<_, AnswerDetail>("SELECT * FROM answers WHERE question_uuid = $1")
            .bind(uuid)
            .fetch_all(self.db.as_ref())
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
    }
}
