pub use self::error::{Error, Result};

mod api;
mod db;
mod entities;
mod error;
mod model;
mod server;

use api::routes::books::books_routes;
use axum::Router;
use model::ModelManager;
use server::cors::set_cors;
use tracing::info;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        /* .with_target(false) */
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // Initialize the ModelManager
    let model_manager = match ModelManager::new().await {
        Ok(model_manager) => model_manager,
        Err(e) => {
            panic!("Error initializing ModelManager: {:?}", e);
        }
    };

    // Initialize Cors
    let cors = set_cors();

    // Initialize the routes
    let mirabel_routes: Router = Router::new()
        .nest("/api/v0/", books_routes(model_manager.clone()))
        .layer(cors);

    // Start the Axum server
    match server::run(mirabel_routes).await {
        Ok(res) => info!("{:?}", res),
        Err(e) => {
            panic!("Error starting server: {:?}", e);
        }
    };
}
