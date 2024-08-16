use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    InternalServerError,
    Unathorized,
    MissingEnvVar(String),

    // Parse Error
    ParseError(String),
    MissingFields(String),

    // Database Error
    DbError(sea_orm::error::DbErr),

    // External API Error
    ExternalApiError(String),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - Error - {self:?}", "INTO_RESPONSE");

        /* (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response() */
        match self {
            Error::NotFound => (StatusCode::NOT_FOUND, "Not Found").into_response(),
            Error::InternalServerError => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
            }
            Error::Unathorized => (StatusCode::UNAUTHORIZED, "Unauthorized").into_response(),
            Error::MissingEnvVar(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            Error::ParseError(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Error::MissingFields(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            Error::DbError(err) => {
                (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response()
            }
            Error::ExternalApiError(msg) => (StatusCode::BAD_GATEWAY, msg).into_response(),
        }
    }
}
