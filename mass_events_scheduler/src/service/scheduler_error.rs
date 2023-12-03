
#[derive(Debug)]
pub enum SchedulerError{
    SQLError(sqlx::Error),
    CronError(cron::error::Error),
}

impl From<sqlx::Error> for SchedulerError {
    fn from(value: sqlx::Error) -> Self {
        Self::SQLError(value)
    }
}

impl From<cron::error::Error> for SchedulerError {
    fn from(value: cron::error::Error) -> Self {
        Self::CronError(value)
    }
}