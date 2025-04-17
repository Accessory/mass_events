use mass_events_utils::validate_trait::Validate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow, ToSchema)]
pub struct Queue {
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueueRequest {
    pub queue: String,
}

impl QueueRequest {
    pub fn new(queue: String) -> Self {
        Self { queue }
    }
}

impl Validate for QueueRequest {
    fn is_valid(&self) -> bool {
        for c in self.queue.chars() {
            if !c.is_alphanumeric() && c != '_' {
                return false;
            }
        }
        true
    }
}
