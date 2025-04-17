use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{self, State},
    http::StatusCode,
    routing::{delete, get, post},
};
use mass_events_process_runner_models::queue::{Queue, QueueRequest};
use mass_events_utils::{return_utils::log_return_internal_server_error, validate_trait::Validate};

use crate::service::queue_service::QueueService;

pub fn new_router(queue_service: Arc<QueueService>) -> Router {
    Router::new()
        .route("/list-queues", get(get_list_queues))
        .route("/create-queue", post(post_create_queue))
        .route("/delete_delete_queue", delete(delete_delete_queue))
        .with_state(queue_service)
}

#[utoipa::path(
    get,
    path = "/list-queues",
    responses(
        (status = 200, description = "List queues.", body = Vec<Queue>),
    ),
    context_path = "/queues"
)]
#[axum::debug_handler]
pub async fn get_list_queues(
    State(queue_service): State<Arc<QueueService>>,
) -> Result<Json<Vec<Queue>>, StatusCode> {
    match queue_service.list_all_queues().await {
        Ok(queues) => Ok(Json(queues)),
        Err(err) => Err(log_return_internal_server_error(err)),
    }
}

#[utoipa::path(
    post,
    path = "/create-queue",
    request_body = QueueRequest,
    responses(
        (status = 200, description = "Create a new queue"),
    ),
    context_path = "/queues"
)]
#[axum::debug_handler]
pub async fn post_create_queue(
    State(queue_service): State<Arc<QueueService>>,
    extract::Json(request): extract::Json<QueueRequest>,
) -> Result<(), StatusCode> {
    if !request.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let does_exists = match queue_service.has_queue(&request.queue).await {
        Ok(result) => result,
        Err(err) => {
            return Err(log_return_internal_server_error(err));
        }
    };
    if does_exists {
        // return HttpResponse::Conflict().body("Queue already exists.");
        return Err(StatusCode::CONFLICT);
    }

    if let Err(err) = queue_service.create_new_queue(&request.queue).await {
        return Err(log_return_internal_server_error(err.as_ref()));
    }

    Ok(())
}

#[utoipa::path(
    delete,
    path = "/delete-queue",
    request_body = QueueRequest,
    responses(
        (status = 200, description = "Create a new queue"),
    ),
    context_path = "/queues"
)]
#[axum::debug_handler]
pub async fn delete_delete_queue(
    State(queue_service): State<Arc<QueueService>>,
    extract::Json(request): extract::Json<QueueRequest>,
) -> Result<(), StatusCode> {
    if !request.is_valid() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let does_exists = match queue_service.has_queue(&request.queue).await {
        Ok(result) => result,
        Err(err) => {
            return Err(log_return_internal_server_error(err));
        }
    };

    if does_exists {
        return Err(StatusCode::CONFLICT);
    }

    if let Err(err) = queue_service.create_new_queue(&request.queue).await {
        return Err(log_return_internal_server_error(err.as_ref()));
    }

    Ok(())
}
