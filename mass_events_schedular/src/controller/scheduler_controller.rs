use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use tracing::warn;

use crate::{
    entities::scheduler_entity::{CreateScheduleRequest, Schedule, NextScheduleResponse},
    service::{
        scheduler_error::SchedulerError::{CronError, SQLError},
        scheduler_service::SchedulerService,
    },
};

pub fn new_router(scheduler_service: Arc<SchedulerService>) -> Router {
    Router::new()
        .route("/new", post(create_new_schedule))
        .route("/list", get(list_schedules))
        .route("/next", get(next_task))
        .with_state(scheduler_service)
}

#[utoipa::path(
    post,
    path = "/new",
    request_body = CreateScheduleRequest,
    responses(
        (status = 200, description = "Create new schedule", body = Schedule),
    ),
    context_path = "/scheduler"
)]
#[axum::debug_handler]
pub async fn create_new_schedule(
    State(scheduler_service): State<Arc<SchedulerService>>,
    extract::Json(req): extract::Json<CreateScheduleRequest>,
) -> Result<Json<Schedule>, StatusCode> {
    match scheduler_service.create_new_schedule(&req).await {
        Ok(status) => Ok(Json(status)),
        Err(err) => match err {
            SQLError(err) => {
                warn!("Sql error: {err}");
                Err(StatusCode::INTERNAL_SERVER_ERROR)
            }
            CronError(err) => {
                warn!("Cron error: {err}; Cron Line: {}", req.cron_line);
                Err(StatusCode::BAD_REQUEST)
            }
        },
    }
}

#[utoipa::path(
    get,
    path = "/list",
    responses(
        (status = 200, description = "List all schedules", body = Vec<Schedule>),
    ),
    context_path = "/scheduler"
)]
#[axum::debug_handler]
pub async fn list_schedules(
    State(scheduler_service): State<Arc<SchedulerService>>,
) -> Result<Json<Vec<Schedule>>, StatusCode> {
    match scheduler_service.list_all_schedules().await {
        Ok(status) => Ok(Json(status)),
        Err(err) => {
            warn!("Failed run path: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

#[utoipa::path(
    get,
    path = "/next",
    responses(
        (status = 200, description = "Get the next task.", body = NextScheduleResponse),
    ),
    context_path = "/scheduler"
)]
#[axum::debug_handler]
pub async fn next_task(
    State(scheduler_service): State<Arc<SchedulerService>>,
) -> Result<Json<NextScheduleResponse>, StatusCode> {
    match scheduler_service.get_next_task().await {
        Ok(optional_scheduler) => match optional_scheduler {
            Some(scheduler) => Ok(Json(scheduler)),
            None => Err(StatusCode::NOT_FOUND),
        },
        Err(err) => {
            warn!("Failed run path: {err}");
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}
