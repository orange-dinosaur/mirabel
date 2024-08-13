use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub type Result<T> = core::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    NotFound,
    InternalServerError,

    // Database Error
    DbError(sea_orm::error::DbErr),
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        println!("--> {:<12} - Error - {self:?}", "INTO_RESPONSE");

        (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
    }
}
