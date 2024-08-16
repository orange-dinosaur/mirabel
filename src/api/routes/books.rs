use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};
use chrono::Utc;
use sea_orm::{
    entity::prelude::Uuid, ActiveModelTrait, ActiveValue, ColumnTrait, EntityTrait, ModelTrait,
    QueryFilter,
};

use crate::{
    entities::books::{self},
    error::Result,
    model::{
        books::{BookFull, BookToSave, BookToUpdate, UserBooks},
        books_api::BooksApiResponse,
        ModelManager,
    },
    Error,
};

pub fn books_routes(model_manager: ModelManager) -> Router {
    Router::new()
        .route("/books", post(save_book))
        .route("/books/:user_id", get(get_user_books))
        .route("/books/:user_id/:id", post(update_book))
        .route("/books/:user_id/:id", delete(delete_book))
        .with_state(model_manager)
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
    let book = book.insert(model_manager.db()).await;
    match book {
        Ok(book) => Ok(format!("Book saved: {:?}", book)),
        Err(e) => Err(Error::DbError(e)),
    }
}

async fn get_user_books(
    State(model_manager): State<ModelManager>,
    Path(user_id): Path<String>,
) -> Result<String> {
    println!("--> {:<12} - get_user_books - ", "GET");

    let db_res = books::Entity::find()
        .filter(books::Column::UserId.eq(user_id.clone()))
        .all(model_manager.db())
        .await;

    let books = if let Ok(books) = db_res {
        books
    } else {
        return Err(Error::InternalServerError);
    };

    // loop through the books and get the full book info from the external API
    let mut user_books = UserBooks::from_user_id(user_id.clone());
    for book in books.iter() {
        let book_id = book.book_id.clone();

        let external_books_api_url = match std::env::var("EXTERNAL_BOOKS_API_URL") {
            Ok(external_books_api_url) => external_books_api_url,
            Err(_) => {
                return Err(Error::MissingEnvVar(
                    "missing env var: EXTERNAL_BOOKS_API_URL".to_string(),
                ));
            }
        };
        let url = external_books_api_url + "/" + &book_id;

        let res = reqwest::get(&url).await;
        match res {
            Ok(res) => {
                let book_api = res.json::<BooksApiResponse>().await;
                match book_api {
                    Ok(book_api) => {
                        user_books.add_book(BookFull::from_db_and_api(book.clone(), book_api)?);
                    }
                    Err(e) => {
                        return Err(Error::ParseError(e.to_string()));
                    }
                }
            }
            Err(e) => {
                return Err(Error::ExternalApiError(e.to_string()));
            }
        }
    }

    Ok(format!("User Books: {:?}", user_books))
}

async fn update_book(
    State(model_manager): State<ModelManager>,
    Path((user_id, id)): Path<(String, String)>,
    Json(book_to_update): Json<BookToUpdate>,
) -> Result<String> {
    println!("--> {:<12} - update_book - ", "UPDATE");

    // check if the id can be parsed into a Uuid
    let id_to_search = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Error::ParseError("Invalid id".to_string())),
    };

    let db_res = books::Entity::find_by_id(id_to_search)
        .one(model_manager.db())
        .await;
    let book = if let Ok(book) = db_res {
        book
    } else {
        return Err(Error::DbError(db_res.unwrap_err()));
    };

    match book.clone() {
        None => return Err(Error::NotFound),
        Some(b) => {
            // check if the user owner of the book is the same one of the call
            if b.user_id != user_id {
                return Err(Error::Unathorized);
            }
        }
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

    let book = book.update(model_manager.db()).await;
    match book {
        Ok(book) => Ok(format!("Book updated: {:?}", book)),
        Err(e) => Err(Error::DbError(e)),
    }
}

async fn delete_book(
    State(model_manager): State<ModelManager>,
    Path((user_id, id)): Path<(String, String)>,
) -> Result<String> {
    println!("--> {:<12} - delete_book - ", "DELETE");

    // check if the id can be parsed into a Uuid
    let id_to_search = match Uuid::parse_str(&id) {
        Ok(id) => id,
        Err(_) => return Err(Error::ParseError("Invalid id".to_string())),
    };

    let book = books::Entity::find_by_id(id_to_search)
        .one(model_manager.db())
        .await;
    let book = match book {
        Ok(book) => {
            if let Some(book) = book {
                book
            } else {
                return Err(Error::NotFound);
            }
        }
        Err(e) => return Err(Error::DbError(e)),
    };

    // check if the user owner of the book is the same one of the call
    if book.user_id != user_id {
        return Err(Error::Unathorized);
    }

    let res = book.delete(model_manager.db()).await;
    match res {
        Ok(res) => Ok(format!("Book deleted result: {:?}", res)),
        Err(e) => Err(Error::DbError(e)),
    }
}
