use std::sync::Arc;

use serde_json::json;
use sqlx::{Error, Pool, Postgres};
use uuid::Uuid;

use crate::{models::process::Process, utils::db_utils::{self, TransactionObject}};

use super::queue_service::QueueService;

pub struct ProcessService {
    pool: Pool<Postgres>,
    queue_service: Arc<QueueService>,
}

pub enum ProcessError {
    QueueNotFound,
    SQLError(Error),
}

impl std::convert::From<sqlx::Error> for ProcessError {
    fn from(value: sqlx::Error) -> Self {
        ProcessError::SQLError(value)
    }
}

impl ProcessService {
    pub fn new(_pool: Pool<Postgres>, queue_service: Arc<QueueService>) -> Self {
        Self {
            pool: _pool,
            queue_service,
        }
    }

    pub async fn get_process(&self, queue: &str) -> Result<Option<Process>, ProcessError> {
        self.does_queue_exists(queue).await?;

        let closure =
        async move |tx: TransactionObject| -> Result<Option<Process>, sqlx::Error> {
            let conn = tx.get_connection_ref();
            // let mut conn = unsafe { ptr::read_volatile(tx) };
            let table_info_name = format!("queues.{queue}_info");
            sqlx::query(&format!("LOCK TABLE {table_info_name}"))
                .execute(conn.as_mut())
                .await?;

            let position: serde_json::Value = sqlx::query_scalar(&format!(
                "select value from {table_info_name} where key = 'position'"
            ))
            .fetch_one(conn.as_mut())
            .await?;

            let result: Option<Process> = sqlx::query_as(&format!(
                "Select * from queues.{queue}_queue WHERE position > $1 order by position LIMIT 1"
            ))
            .bind(position.as_i64().unwrap())
            .fetch_optional(conn.as_mut())
            .await?;

            if let Some(process) = &result {
                sqlx::query(&format!(
                    "UPDATE {table_info_name} SET value= $1 WHERE key = 'position'"
                ))
                .bind(json!(process.position))
                .execute(conn.as_mut())
                .await?;
            }

            Ok(result)
        };

        let rtn = db_utils::in_transaction(&self.pool, closure).await?;

        Ok(rtn)
    }

    pub async fn add_process(
        &self,
        queue: &str,
        process: &serde_json::Value,
    ) -> Result<(), ProcessError> {
        self.does_queue_exists(queue).await?;

        if let Err(err) = sqlx::query(&format!(
            "INSERT INTO queues.{}_queue(id, context) VALUES ($1, $2);",
            queue
        ))
        .bind(Uuid::new_v4())
        .bind(process)
        .execute(&self.pool)
        .await
        {
            return Err(ProcessError::SQLError(err));
        }

        Ok(())
    }

    pub async fn does_queue_exists(&self, queue: &str) -> Result<(), ProcessError> {
        match self.queue_service.has_queue(queue).await {
            Ok(has_queue) => {
                if !has_queue {
                    return Err(ProcessError::QueueNotFound);
                }
            }
            Err(err) => return Err(ProcessError::SQLError(err)),
        };
        Ok(())
    }
}
