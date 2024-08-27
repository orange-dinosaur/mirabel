pub mod cors;

use axum::Router;
use tracing::{error as tracing_error, info};

use crate::error::{self, Result};

pub async fn run(routes: Router) -> Result<String> {
    let port = match std::env::var("PORT") {
        Ok(port) => port,
        Err(_) => {
            return Err(error::Error::MissingEnvVar("PORT".to_string()));
        }
    };
    let listener = match tokio::net::TcpListener::bind("0.0.0.0:".to_string() + &port).await {
        Ok(listener) => listener,
        Err(e) => {
            tracing_error!("Error binding to port {:?}: {:?}", port, e);
            return Err(error::Error::InternalServerError);
        }
    };

    match axum::serve(listener, routes).await {
        Ok(_) => {
            info!("Server started");
            Ok("Server started".to_string())
        }
        Err(e) => {
            tracing_error!("Error starting server: {:?}", e);
            Err(error::Error::InternalServerError)
        }
    }
}
