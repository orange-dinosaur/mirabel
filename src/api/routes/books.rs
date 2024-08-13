use axum::{extract::State, routing::post, Json, Router};
use chrono::Utc;
use sea_orm::{entity::prelude::Uuid, ActiveModelTrait, ActiveValue};
use serde::Deserialize;

use crate::{
    entities::books::{self},
    error::Result,
    model::ModelManager,
};

pub fn books_routes(model_manager: ModelManager) -> Router {
    Router::new()
        .route("/books", post(save_book))
        .with_state(model_manager)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookToSave {
    book_id: String,
    user_id: String,
    reading_status: String,
    book_type: String,
    tags: Vec<String>,
    rating: f32,
    notes: String,
    library_id: String,
}

async fn save_book(
    State(model_manager): State<ModelManager>,
    Json(book_to_save): Json<BookToSave>,
) -> Result<String> {
    println!("--> {:<12} - save_book - ", "POST");

    let book = books::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        created_at: ActiveValue::set(Some(Utc::now().naive_utc())),
        updated_at: ActiveValue::NotSet,
        book_id: ActiveValue::Set(book_to_save.book_id),
        user_id: ActiveValue::Set(book_to_save.user_id),
        reading_status: ActiveValue::Set(Some(book_to_save.reading_status)),
        book_type: ActiveValue::Set(Some(book_to_save.book_type)),
        tags: ActiveValue::Set(Some(book_to_save.tags)),
        rating: ActiveValue::Set(Some(book_to_save.rating as f64)),
        notes: ActiveValue::Set(Some(book_to_save.notes)),
        library_id: ActiveValue::Set(Some(book_to_save.library_id)),
    };

    // Save the book in the database
    let book: books::Model = book.insert(model_manager.db()).await.unwrap();

    Ok(format!("Book saved: {:?}", book))
}
