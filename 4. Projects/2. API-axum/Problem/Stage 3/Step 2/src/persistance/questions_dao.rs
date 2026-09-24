use std::sync::Arc;
use async_trait::async_trait;
use di::injectable;

use crate::models::{DBError, Question, QuestionDetail};
use sqlx::types::Uuid;
use sqlx::{Executor, PgPool};
use std::str::FromStr;

#[async_trait]
pub trait QuestionsDao: Send + Sync {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

#[injectable(QuestionsDao)]
pub struct QuestionsDaoImpl {
    db: Arc<PgPool>,
}

impl QuestionsDaoImpl {
    pub fn new(db: Arc<PgPool>) -> Self {
        Self { db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        sqlx::query_as::<_, QuestionDetail>("INSERT INTO questions (title, description) VALUES ($1, $2) RETURNING *")
            .bind(question.title)
            .bind(question.description)
            .fetch_one(self.db.as_ref())
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let uuid = Uuid::from_str(&question_uuid).map_err(|err| DBError::InvalidUUID(question_uuid))?;

        sqlx::query("DELETE FROM questions WHERE question_uuid = $1")
            .bind(uuid)
            .execute(self.db.as_ref())
            .await
            .map(|_| ())
            .map_err(|e| DBError::Other(Box::new(e)))
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        sqlx::query_as::<_, QuestionDetail>("SELECT * FROM questions")
            .fetch_all(self.db.as_ref())
            .await
            .map_err(|e| DBError::Other(Box::new(e)))
    }
}
