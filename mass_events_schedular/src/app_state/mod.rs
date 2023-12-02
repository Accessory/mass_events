use std::{collections::HashMap, sync::Arc};

use sqlx::{Pool, Postgres};
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};
use tracing::info;

use crate::{entities::scheduler_entity::Schedule, service::scheduler_service::SchedulerService};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Postgres>,
    pub scheduler: tokio_cron_scheduler::JobScheduler,
    pub job_db_map: HashMap<uuid::Uuid, uuid::Uuid>,
    pub db_job_map: HashMap<uuid::Uuid, uuid::Uuid>,
}
impl AppState {
    pub async fn new_with(db: Pool<Postgres>) -> Self {
        let job_db_map = HashMap::new();
        let db_job_map = HashMap::new();

        let scheduler = JobScheduler::new()
            .await
            .expect("Could not initlize JobScheduler.");

        Self {
            db,
            scheduler,
            job_db_map,
            db_job_map,
        }
    }
}

pub async fn init_scheduler(app_state: Arc<RwLock<AppState>>, scheduler_service: Arc<SchedulerService>) {
    let mut borrow = app_state.write().await;
    let schedules: Vec<Schedule> = sqlx::query_as("SELECT * FROM public.schedules")
        .fetch_all(&borrow.db)
        .await
        .expect("Could not get the stored Schedules");

    for schedule in schedules {
        let new_job = create_new_job(&schedule, &scheduler_service);
        borrow.job_db_map.insert(new_job.guid(), schedule.id);
        borrow.db_job_map.insert(schedule.id, new_job.guid());
        borrow
            .scheduler
            .add(new_job)
            .await
            .expect("Failed to add job to scheduler.");
    }

    borrow
        .scheduler
        .start()
        .await
        .expect("Failed to start scheduler.");
}

fn create_new_job(schedule: &Schedule, scheduler_service: &Arc<SchedulerService>) -> Job {
    let ss1 = scheduler_service.clone();
    Job::new_async(schedule.cron_line.as_str(), move |uuid, mut l| {
        let ss2 = ss1.clone();
        Box::pin(async move {
            let next_tick = l
                .next_tick_for_job(uuid)
                .await
                .expect("Could not get next tick from job")
                .expect("Next tick was empty.");
            info!("Running job with id: {uuid}. Next tick at: {next_tick}");
            let schedule: Schedule = ss2.get_schedule_from_job_id(&uuid).await.expect("Failed to get Schedule.").expect("Schedule not found by JobId: {uuid}");
            info!("Running Schedule with id {}. Executing the command {}", schedule.id, schedule.command);
        })
    })
    .expect("Failed to create job")
}
