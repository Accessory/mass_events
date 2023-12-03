use askama::Template;
use mass_events_process_runner_models::queue::Queue;
use mass_events_utils::db_utils;
use sqlx::{Error, Pool, Postgres};

use crate::{
    templates::sql::{
        create_queue_template::CreateQueueTemplate, delete_queue_template::DeleteQueueTemplate,
    },
};

pub struct QueueService {
    pool: Pool<Postgres>,
}

impl QueueService {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool: pool }
    }

    pub async fn list_all_queues(&self) -> Result<Vec<Queue>, Error> {
        let queues_result: Vec<Queue> = sqlx::query_as("SELECT name FROM public.queues")
            .fetch_all(&self.pool)
            .await?;

        Ok(queues_result)
    }

    pub async fn has_queue(&self, name: &str) -> Result<bool, Error> {
        let queues_result: Option<i32> =
            sqlx::query_scalar("SELECT 1 FROM public.queues WHERE name = $1")
                .bind(name)
                .fetch_optional(&self.pool)
                .await?;

        Ok(queues_result.is_some())
    }

    pub async fn create_new_queue(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sql_template = CreateQueueTemplate::new(name);
        let sql = sql_template.render()?;

        db_utils::execute_transaction(&sql, &self.pool).await?;

        Ok(())
    }

    pub async fn delete_queue(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let sql_template = DeleteQueueTemplate::new(name);
        let sql = sql_template.render()?;

        db_utils::execute_transaction(&sql, &self.pool).await?;

        Ok(())
    }
}
