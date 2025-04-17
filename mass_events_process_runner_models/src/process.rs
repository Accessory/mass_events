use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Process {
    pub id: uuid::Uuid,
    pub context: serde_json::Value,
    pub position: i64,
}

#[derive(ToSchema, IntoParams, Serialize, Deserialize, Debug)]
pub struct AddProcessToQueueRequest {
    pub queue: String,
    pub process: serde_json::Value,
}

impl AddProcessToQueueRequest {
    pub fn new(queue: String, process: serde_json::Value) -> Self {
        Self { queue, process }
    }
}
