use crate::controller;
use crate::entities;
use utoipa::OpenApi;

// OpenApi
#[derive(OpenApi)]
#[openapi(
    info(description = "Process Runner"),
    paths(
        controller::scheduler_controller::create_new_schedule,
        controller::scheduler_controller::list_schedules,
        controller::scheduler_controller::next_task,
    ),
    tags(
        (name = "controller::scheduler_controller", description = "Main scheduler controller.")
    ),
    components(schemas(
            entities::scheduler_entity::Schedule,
            entities::scheduler_entity::CreateScheduleRequest,
            entities::scheduler_entity::NextScheduleResponse,
        ))
)]
pub struct ApiDoc;
