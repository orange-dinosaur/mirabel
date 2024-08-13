pub use self::error::{Error, Result};

mod api;
mod db;
mod entities;
mod error;
mod model;

use api::routes::books::books_routes;
use axum::{
    extract::{Path, Query},
    routing::get,
    Router,
};
use model::ModelManager;
use serde::Deserialize;

#[tokio::main]
async fn main() {
    // Initialize the ModelManager
    let model_manager = ModelManager::new().await.unwrap();

    let mirabel_routes: Router = Router::new()
        .nest("/api/v0/", hello_routes(model_manager.clone()))
        .nest("/api/v0/", books_routes(model_manager.clone()));

    // region - Start Server (run our app with hyper, listening globally on port 3000)
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, mirabel_routes).await.unwrap();
    // endregion - Start Server
}

// region - Hello World
pub fn hello_routes(model_manager: ModelManager) -> Router {
    Router::new()
        .route("/hello", get(hello_world))
        .route("/hello2/:name", get(hello_world2))
        .with_state(model_manager)
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: String,
}

async fn hello_world(Query(params): Query<HelloParams>) -> String {
    println!("--> {:<12} - hello_world - {:?}", "GET", params);

    let name = &params.name;
    format!("Hello, {}!", name)
}

async fn hello_world2(Path(name): Path<String>) -> String {
    println!("--> {:<12} - hello_world2 - {:?}", "GET", name);

    format!("Hello, {}!", name)
}
// endregion - Hello World
