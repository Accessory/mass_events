use std::error::Error;

use hyper::StatusCode;
use tracing::warn;

#[inline(always)]
pub fn log_return_internal_server_error(err: impl Error) -> StatusCode {
    warn!("Error: {err:?}");
    StatusCode::INTERNAL_SERVER_ERROR
}
