use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use strum_macros::AsRefStr;

use crate::api::response::Response as ApiResponse;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Clone, Debug, Serialize, AsRefStr)]
#[serde(tag = "type", content = "data")]
pub enum Error {
    NotFound,
    InternalServerError,
    Unathorized,
    MissingEnvVar(String),

    // Parse Error
    ParseError(String),
    MissingFields(String),

    // Database Error
    DbError(String),

    // External API Error
    ExternalApiError(String),
}

#[derive(Debug, AsRefStr)]
#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
pub enum ClientError {
    INTERNAL_SERVER_ERROR,
    NOT_FOUND,
    UNAUTHORIZED,
    BAD_REQUEST,
    BAD_GATEWAY,
}

impl Error {
    pub fn client_status_and_error(&self) -> (StatusCode, ClientError) {
        match self {
            Self::NotFound => (StatusCode::NOT_FOUND, ClientError::NOT_FOUND),
            Self::InternalServerError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_SERVER_ERROR,
            ),
            Self::Unathorized => (StatusCode::UNAUTHORIZED, ClientError::UNAUTHORIZED),
            Self::MissingEnvVar(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_SERVER_ERROR,
            ),

            Self::ParseError(_) => (StatusCode::BAD_REQUEST, ClientError::BAD_REQUEST),
            Self::MissingFields(_) => (StatusCode::BAD_REQUEST, ClientError::BAD_REQUEST),

            Self::DbError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ClientError::INTERNAL_SERVER_ERROR,
            ),

            Self::ExternalApiError(_) => (StatusCode::BAD_GATEWAY, ClientError::BAD_GATEWAY),
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - Error - {self:?}", "INTO_RESPONSE");

        // return a json response with the error
        let (status, error) = self.client_status_and_error();

        let res_body = ApiResponse::<String>::new_error(
            status.as_u16(),
            Some(error.as_ref().to_string()),
            None,
            None,
        )
        .to_json();

        Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(axum::body::Body::from(res_body.to_string()))
            .unwrap()
    }
}
