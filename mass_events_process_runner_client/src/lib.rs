use std::fmt::Display;

use mass_events_process_runner_models::{
    process::AddProcessToQueueRequest,
    queue::{Queue, QueueRequest},
};
use reqwest::StatusCode;
use serde_json::Value;

#[derive(Debug)]
pub enum ProcessRunnerClientError {
    SendingError(reqwest::Error),
    ReturnStatusError(StatusCode),
}

impl Display for ProcessRunnerClientError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for ProcessRunnerClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(self)
    }
}

impl From<StatusCode> for ProcessRunnerClientError {
    fn from(value: StatusCode) -> Self {
        Self::ReturnStatusError(value)
    }
}

impl From<reqwest::Error> for ProcessRunnerClientError {
    fn from(value: reqwest::Error) -> Self {
        Self::SendingError(value)
    }
}

pub struct ProcessRunnerClient {
    base: String,
    client: reqwest::Client,
}

impl ProcessRunnerClient {
    pub fn new(base: &str) -> Self {
        Self {
            base: base.into(),
            client: reqwest::Client::new(),
        }
    }
    pub async fn post_get_process(&self, queue: &str) -> Result<Value, ProcessRunnerClientError> {
        let body = QueueRequest::new(queue.into());
        let result = self
            .client
            .post(format!("{}/process/get_process", &self.base))
            .json(&body)
            .send()
            .await?;

        match result.status() {
            StatusCode::OK => {
                let content: Value = result.json().await?;
                Ok(content)
            }
            status_code => Err(status_code.into()),
        }
    }

    pub async fn post_add_process(
        &self,
        queue: &str,
        value: serde_json::Value,
    ) -> Result<(), ProcessRunnerClientError> {
        let body = AddProcessToQueueRequest::new(queue.into(), value);

        let result = self
            .client
            .post(format!("{}/process/add_process", self.base))
            .json(&body)
            .send()
            .await?;

        match result.status() {
            StatusCode::OK => Ok(()),
            status_code => Err(status_code.into()),
        }
    }

    pub async fn get_list_queues(&self) -> Result<Vec<Queue>, ProcessRunnerClientError> {
        let result = self
            .client
            .get(format!("{}/queues/list-queues", &self.base))
            .send()
            .await?;

        match result.status() {
            StatusCode::OK => {
                let content = result.json().await?;
                Ok(content)
            }
            status_code => Err(status_code.into()),
        }
    }
}
