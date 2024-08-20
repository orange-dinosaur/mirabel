use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response<T: Serialize> {
    pub status: u16,
    pub error: bool,
    pub error_type: Option<String>,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn new_error(
        status: u16,
        error_type: Option<String>,
        message: Option<String>,
        data: Option<T>,
    ) -> Self {
        Self {
            status,
            error: true,
            error_type,
            message,
            data,
        }
    }

    pub fn new_success(status: u16, message: Option<String>, data: Option<T>) -> Self {
        Self {
            status,
            error: false,
            error_type: None,
            message,
            data,
        }
    }

    // return a serde_json object
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::json!(self)
    }
}
