use std::sync::Arc;

use axum::{
    extract::{self, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    routing::post,
    Json, Router,
};
use serde_json::json;

use crate::{
    models::{process::AddProcessToQueueRequest, queue::QueueRequest},
    service::process_service::{ProcessError, ProcessService},
    utils::return_utils::log_return_internal_server_error,
};

pub fn new_router(process_service: Arc<ProcessService>) -> Router {
    Router::new()
        .route("/add_process", post(post_add_process))
        .route("/get_process", post(post_get_process))
        .with_state(process_service)
}

#[utoipa::path(
    post,
    path = "/add_process",
    request_body = AddProcessToQueueRequest,
    responses(
        (status = 200, description = "Add a process."),
    ),
    context_path = "/process"
)]
#[axum::debug_handler]
pub async fn post_add_process(
    State(process_service): State<Arc<ProcessService>>,
    extract::Json(request): extract::Json<AddProcessToQueueRequest>,
) -> Result<(), StatusCode> {
    if let Err(err) = process_service
        .add_process(&request.queue, &request.process)
        .await
    {
        match err {
            ProcessError::QueueNotFound => {
                return Err(StatusCode::NOT_FOUND);
            }
            ProcessError::SQLError(err) => {
                return Err(log_return_internal_server_error(err));
            }
        };
    }

    Ok(())
}

#[utoipa::path(
    post,
    path = "/get_process",
    request_body = QueueRequest,
    responses(
        (status = 200, description = "Get a process"),
    ),
    context_path = "/process"
)]
#[axum::debug_handler]
pub async fn post_get_process(
    State(process_service): State<Arc<ProcessService>>,
    extract::Json(request): extract::Json<QueueRequest>,
) -> Result<Response, StatusCode> {
    let rtn = match process_service.get_process(&request.queue).await {
        Err(err) => {
            match err {
                ProcessError::QueueNotFound => {
                    return Err(StatusCode::NOT_FOUND);
                }
                ProcessError::SQLError(err) => {
                    return Err(log_return_internal_server_error(err));
                }
            };
        }
        Ok(rtn) => rtn,
    };

    match rtn {
        Some(send) => Ok(Json(send.context).into_response()),
        None => Ok(Json(json!({})).into_response()),
    }
    // Ok(Json(json!({})).into_response())
}
