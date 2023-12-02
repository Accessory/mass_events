use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Schedule {
    pub id: Uuid,
    pub cron_line: String,
    pub command: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct NextScheduleResponse {
    pub id: Uuid,
    pub cron_line: String,
    pub command: String,
    pub created_at: DateTime<Utc>,
    pub modified_at: DateTime<Utc>,
    pub next: DateTime<Utc>,
}
impl NextScheduleResponse {
    pub(crate) fn with(schedule: Schedule, job_and_tick: u64) -> Self {
        let next = DateTime::<Utc>::from_timestamp(job_and_tick as i64, 0).expect("Could not create a new DateTime<Utc>");
        Self {
            id: schedule.id,
            cron_line: schedule.cron_line,
            command: schedule.command,
            created_at: schedule.created_at,
            modified_at: schedule.modified_at,
            next,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct CreateScheduleRequest {
    pub cron_line: String,
    pub command: String,
}
