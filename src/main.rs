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

#[tokio::main]
async fn main() {
    // Initialize the ModelManager
    let model_manager = match ModelManager::new().await {
        Ok(model_manager) => model_manager,
        Err(e) => {
            panic!("Error initializing ModelManager: {:?}", e);
        }
    };

    // Initialize the routes
    let mirabel_routes: Router =
        Router::new().nest("/api/v0/", books_routes(model_manager.clone()));

    // Start the Axum server
    match server::run(mirabel_routes).await {
        Ok(res) => println!("{:?}", res),
        Err(e) => {
            panic!("Error starting server: {:?}", e);
        }
    };
}
