use axum::{
    extract::{Path, Query, State},
    routing::{delete, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{
    entity::prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, QueryFilter,
};
use serde::Deserialize;

use crate::{
    entities::books::{self},
    error::Result,
    model::ModelManager,
};

pub fn books_routes(model_manager: ModelManager) -> Router {
    Router::new()
        .route("/books", post(save_book))
        .route("/books/:id", post(update_book))
        .route("/books/:id", delete(delete_book))
        .with_state(model_manager)
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookToSave {
    book_id: String,
    user_id: String,
    reading_status: Option<String>,
    book_type: Option<String>,
    tags: Option<Vec<String>>,
    rating: Option<f32>,
    notes: Option<String>,
    library_id: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct BookToUpdate {
    reading_status: Option<String>,
    book_type: Option<String>,
    tags: Option<Vec<String>>,
    rating: Option<f32>,
    notes: Option<String>,
    library_id: Option<String>,
}

#[derive(Debug, Deserialize)]
struct BookParams {
    user_id: String,
}

async fn save_book(
    State(model_manager): State<ModelManager>,
    Json(book_to_save): Json<BookToSave>,
) -> Result<String> {
    println!("--> {:<12} - save_book - ", "POST");

    let mut book = books::ActiveModel {
        id: ActiveValue::Set(Uuid::new_v4()),
        created_at: ActiveValue::set(Some(Utc::now().naive_utc())),
        book_id: ActiveValue::Set(book_to_save.book_id),
        user_id: ActiveValue::Set(book_to_save.user_id),
        ..Default::default()
    };

    // check if the optional fields are set and update the active model
    if let Some(reading_status) = book_to_save.reading_status {
        book.reading_status = ActiveValue::Set(Some(reading_status));
    };
    if let Some(book_type) = book_to_save.book_type {
        book.book_type = ActiveValue::Set(Some(book_type));
    };
    if let Some(tags) = book_to_save.tags {
        book.tags = ActiveValue::Set(Some(tags));
    };
    if let Some(rating) = book_to_save.rating {
        book.rating = ActiveValue::Set(Some(rating as f64));
    };
    if let Some(notes) = book_to_save.notes {
        book.notes = ActiveValue::Set(Some(notes));
    };
    if let Some(library_id) = book_to_save.library_id {
        book.library_id = ActiveValue::Set(Some(library_id));
    };

    // Save the book in the database
    let book: books::Model = book.insert(model_manager.db()).await.unwrap();

    Ok(format!("Book saved: {:?}", book))
}

async fn update_book(
    State(model_manager): State<ModelManager>,
    Path(id): Path<String>,
    Query(params): Query<BookParams>,
    Json(book_to_update): Json<BookToUpdate>,
) -> Result<String> {
    println!("--> {:<12} - update_book - ", "UPDATE");

    let book = books::Entity::find_by_id(Uuid::parse_str(&id).unwrap())
        .one(model_manager.db())
        .await
        .unwrap();

    // check if the user owner of the book is the same one of the call
    if book.as_ref().unwrap().user_id != params.user_id {
        // return error
    }

    // transofrm the book into an ActiveModel so it can be updated
    let mut book: books::ActiveModel = book.unwrap().into();

    book.updated_at = ActiveValue::set(Some(Utc::now().naive_utc()));
    // check if the optional fields are set and update the active model accordingly
    if let Some(reading_status) = book_to_update.reading_status {
        book.reading_status = ActiveValue::Set(Some(reading_status));
    };
    if let Some(book_type) = book_to_update.book_type {
        book.book_type = ActiveValue::Set(Some(book_type));
    };
    if let Some(tags) = book_to_update.tags {
        book.tags = ActiveValue::Set(Some(tags));
    };
    if let Some(rating) = book_to_update.rating {
        book.rating = ActiveValue::Set(Some(rating as f64));
    };
    if let Some(notes) = book_to_update.notes {
        book.notes = ActiveValue::Set(Some(notes));
    };
    if let Some(library_id) = book_to_update.library_id {
        book.library_id = ActiveValue::Set(Some(library_id));
    };

    let book = book.update(model_manager.db()).await.unwrap();

    Ok(format!("Book updated: {:?}", book))
}

async fn delete_book(
    State(model_manager): State<ModelManager>,
    Path(id): Path<String>,
    Query(params): Query<BookParams>,
) -> Result<String> {
    println!("--> {:<12} - delete_book - ", "DELETE");

    let res = books::Entity::delete_many()
        .filter(books::Column::Id.eq(Uuid::parse_str(&id).unwrap()))
        .filter(books::Column::UserId.eq(params.user_id.to_string()))
        .exec(model_manager.db())
        .await
        .unwrap();

    Ok(format!("Book deleted: {:?}", res))
}
