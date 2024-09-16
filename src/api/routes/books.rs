use axum::{
    extract::{Path, State},
    routing::{delete, get, post},
    Json, Router,
};

use sea_orm::{
    entity::prelude::Uuid, ActiveModelTrait, ColumnTrait, EntityTrait, ModelTrait, QueryFilter,
};
use tracing::info;

use crate::{
    api::response::Response,
    entities::books::{self},
    error::Result,
    model::{
        books::{BookFull, BookId, BookToSave, BookToUpdate, UserBooks},
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
) -> Result<Json<Response<BookId>>> {
    info!("{:<6} - save_book", "POST");

    let book = book_to_save.to_active_model();

    // Save the book in the database
    let book = book.insert(model_manager.db()).await;
    match book {
        Ok(b) => {
            // return only the id of the book created
            let res = Response::new_success(
                201,
                Some("Book created successfully!".to_string()),
                Some(BookId {
                    id: b.id.to_string(),
                }),
            );
            Ok(Json(res))
        }
        Err(e) => Err(Error::DbError(e.to_string())),
    }
}

async fn get_user_books(
    State(model_manager): State<ModelManager>,
    Path(user_id): Path<String>,
) -> Result<Json<Response<UserBooks>>> {
    info!("{:<6} - get_user_books", "GET");

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
        let external_books_api_key = match std::env::var("EXTERNAL_BOOKS_API_KEY") {
            Ok(external_books_api_key) => external_books_api_key,
            Err(_) => {
                return Err(Error::MissingEnvVar(
                    "missing env var: EXTERNAL_BOOKS_API_KEY".to_string(),
                ));
            }
        };
        let url = external_books_api_url + "/" + &book_id + "?key=" + &external_books_api_key;

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

    let res = Response::new_success(200, None, Some(user_books));
    Ok(Json(res))
}

async fn update_book(
    State(model_manager): State<ModelManager>,
    Path((user_id, id)): Path<(String, String)>,
    Json(book_to_update): Json<BookToUpdate>,
) -> Result<Json<Response<String>>> {
    info!("{:<6} - update_book", "UPDATE");

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
        return Err(Error::DbError(db_res.unwrap_err().to_string()));
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

    let b = book_to_update.to_active_model(book);
    let book = b.update(model_manager.db()).await;
    match book {
        Ok(_) => {
            let res = Response::<String>::new_success(
                200,
                Some("Book updated successfully!".to_string()),
                None,
            );
            Ok(Json(res))
        }
        Err(e) => Err(Error::DbError(e.to_string())),
    }
}

async fn delete_book(
    State(model_manager): State<ModelManager>,
    Path((user_id, id)): Path<(String, String)>,
) -> Result<Json<Response<String>>> {
    info!("{:<6} - delete_book", "DELETE");

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
        Err(e) => return Err(Error::DbError(e.to_string())),
    };

    // check if the user owner of the book is the same one of the call
    if book.user_id != user_id {
        return Err(Error::Unathorized);
    }

    let res = book.delete(model_manager.db()).await;
    match res {
        Ok(_) => {
            let res = Response::<String>::new_success(
                200,
                Some("Book deleted successfully!".to_string()),
                None,
            );
            Ok(Json(res))
        }
        Err(e) => Err(Error::DbError(e.to_string())),
    }
}
