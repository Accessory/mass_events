use tokio::sync::RwLock;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    entities::scheduler_entity::{CreateScheduleRequest, NextScheduleResponse, Schedule},
    utils::u128_to_uuid,
};
use std::{str::FromStr, sync::Arc};

use super::scheduler_error::SchedulerError;

pub struct SchedulerService {
    pub state: Arc<RwLock<AppState>>,
}

impl SchedulerService {
    pub async fn get_next_task(&self) -> Result<Option<NextScheduleResponse>, sqlx::Error> {
        let borrow = self.state.blocking_read();
        let next_tick_list = borrow
            .scheduler
            .context
            .metadata_storage
            .write()
            .await
            .list_next_ticks()
            .await
            .expect("Could not get the next tick list.");

        let next_tick = next_tick_list.first();

        if let Some(job_and_tick) = next_tick {
            let id = job_and_tick.id.as_ref().expect("Could not get job id.");
            let idu128 = u128_to_uuid(id.as_u128());
            let db_id = borrow
                .job_db_map
                .get(&idu128)
                .expect("Uuid not found.");
            let sql_result = self.get_schedule(db_id).await?;
            if sql_result.is_none() {
                return Ok(Option::None);
            }
            Ok(Some(NextScheduleResponse::with(
                sql_result.unwrap(),
                job_and_tick.next_tick,
            )))
        } else {
            Ok(Option::None)
        }
    }

    pub async fn get_schedule_from_job_id(&self, id: &Uuid) -> Result<Option<Schedule>, sqlx::Error> {
        let borrow = self.state.read().await;
        let db_id = borrow.job_db_map.get(id).expect("Job id not found.");
        self.get_schedule(db_id).await
    }

    pub async fn get_schedule(&self, id: &Uuid) -> Result<Option<Schedule>, sqlx::Error> {
        let rtn = sqlx::query_as("SELECT * FROM public.schedules WHERE id = $1")
            .bind(id)
            .fetch_optional(&self.state.read().await.db)
            .await?;
        Ok(rtn)
    }

    pub async fn list_all_schedules(&self) -> Result<Vec<Schedule>, sqlx::Error> {
        let rtn = sqlx::query_as("SELECT * FROM public.schedules")
            .fetch_all(&self.state.read().await.db)
            .await?;
        Ok(rtn)
    }

    pub async fn create_new_schedule(
        &self,
        schedule: &CreateScheduleRequest,
    ) -> Result<Schedule, SchedulerError> {
        let _ = cron::Schedule::from_str(&schedule.cron_line)?;

        let rtn = sqlx::query_as(
            "INSERT INTO public.schedules(cron_line, command) VALUES ($1, $2) returning *;",
        )
        .bind(&schedule.cron_line)
        .bind(&schedule.command)
        .fetch_one(&self.state.blocking_read().db)
        .await?;

        Ok(rtn)
    }

        // pub async fn get_next_schedule(&self) -> Result<Option<Schedule>, sqlx::Error> {
    //     let next_tick_list = self
    //         .state
    //         .scheduler
    //         .context
    //         .metadata_storage
    //         .write()
    //         .await
    //         .list_next_ticks()
    //         .await
    //         .expect("Could not get the next tick list.");

    //     let next_tick = next_tick_list.first();

    //     if let Some(job_and_tick) = next_tick {
    //         let id = job_and_tick.id.as_ref().expect("Could not get job id.");
    //         let idu128 = u128_to_uuid(id.as_u128());
    //         let db_id = self.state.job_db_map.get(&idu128).expect("Uuid not found.");
    //         let sql_result = self.get_schedule(db_id).await?;
    //         Ok(sql_result)
    //     } else {
    //         Ok(Option::None)
    //     }
    // }
}
