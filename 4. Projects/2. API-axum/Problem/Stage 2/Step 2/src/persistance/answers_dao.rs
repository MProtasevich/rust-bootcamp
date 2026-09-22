use crate::models::{postgres_error_codes, Answer, AnswerDetail, DBError};
use sqlx::types::Uuid;
use sqlx::PgPool;
use std::str::FromStr;

pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

pub struct AnswersDaoImpl {
    db: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }
}

impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError> {
        let uuid = Uuid::from_str(answer.question_uuid.as_str())
            .map_err(|err| DBError::InvalidUUID(answer.question_uuid))?;
        sqlx::query_as::<_, AnswerDetail>("INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *")
            .bind(uuid)
            .bind(answer.content)
            .fetch_one(&self.db)
            .await
            .map_err(|e| match e {
                sqlx::Error::Database(error) if error.code().is_some_and(|code| code == postgres_error_codes::FOREIGN_KEY_VIOLATION) => DBError::InvalidUUID(error.message().to_string()),
                _ => DBError::Other(Box::new(e))
            })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::from_str(&answer_uuid).map_err(|err| DBError::InvalidUUID(answer_uuid))?;

        sqlx::query("DELETE FROM answers WHERE answer_uuid = $1")
            .bind(uuid)
            .execute(&self.db)
            .await
            .map(|_| ())
            .map_err(|e| DBError::Other(Box::new(e)))
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let uuid = Uuid::from_str(&question_uuid).map_err(|err| DBError::InvalidUUID(question_uuid))?;

        sqlx::query_as::<_, AnswerDetail>("SELECT * FROM answers WHERE question_uuid = $1")
            .bind(uuid)
            .fetch_all(&self.db)
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
    }
}
