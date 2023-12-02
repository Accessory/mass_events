use crate::controller;
use crate::models;
use axum::response::Redirect;
use utoipa::OpenApi;

// OpenApi
#[derive(OpenApi)]
#[openapi(
    info(description = "Process Runner"),
    paths(
        controller::queue_controller::get_list_queues,
        controller::queue_controller::post_create_queue,
        controller::queue_controller::delete_delete_queue,
        controller::process_controller::post_add_process,
        controller::process_controller::post_get_process,
    ),
    tags(
        (name = "controller::queue_controller", description = "The queue controller with the neccessary endpoints to controll the queues.")
    ),
    components(schemas(
        models::queue::Queue,
        models::queue::QueueRequest,
        models::process::Process,
        models::process::AddProcessToQueueRequest,
        ))
)]
pub struct ApiDoc;

#[utoipa::path(
    get,
    path="/",
    responses(
        (status = 308, description = "Redirect to the swagger-ui.")
    ),
    context_path = ""
)]
pub async fn redirect_to_openapi() -> Redirect {
    Redirect::permanent("/swagger-ui/")
}
