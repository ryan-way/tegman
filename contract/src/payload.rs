use serde::{Deserialize, Serialize};

use crate::{operation::Operation, request::Request};

#[derive(Serialize, Deserialize)]
pub struct RequestPayload {
    pub request: Request,
    pub operation: Operation,
}
