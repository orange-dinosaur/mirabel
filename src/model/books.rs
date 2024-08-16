use serde::Deserialize;

use crate::{
    entities::books::Model,
    error::{Error, Result},
};

use super::books_api::BooksApiResponse;

// region - BookToSave
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookToSave {
    pub book_id: String,
    pub user_id: String,
    pub reading_status: Option<String>,
    pub book_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<f32>,
    pub notes: Option<String>,
    pub library_id: Option<String>,
}
// endregion - BookToSave

// region - BookToUpdate
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BookToUpdate {
    pub reading_status: Option<String>,
    pub book_type: Option<String>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<f32>,
    pub notes: Option<String>,
    pub library_id: Option<String>,
}
// endregion - BookToUpdate

// region - BookFull
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
#[allow(dead_code)]
pub struct BookFull {
    pub id: String,
    pub book_id: String,
    pub title: String,
    pub authors: Vec<String>,
    pub publisher: String,
    pub published_date: String,
    pub description: String,
    pub isbn10: String,
    pub isbn13: String,
    pub page_count: i64,
    pub categories: Vec<String>,
    pub language: String,
    pub cover: String,
    pub reading_status: String,
    pub book_type: String,
    pub tags: Vec<String>,
    pub rating: f32,
    pub notes: String,
    pub library_id: String,
}

impl Default for BookFull {
    fn default() -> Self {
        Self {
            id: "".to_string(),
            book_id: "".to_string(),
            title: "".to_string(),
            authors: vec![],
            publisher: "".to_string(),
            published_date: "".to_string(),
            description: "".to_string(),
            isbn10: "".to_string(),
            isbn13: "".to_string(),
            page_count: 0,
            categories: vec![],
            language: "".to_string(),
            cover: "".to_string(),
            reading_status: "".to_string(),
            book_type: "".to_string(),
            tags: vec![],
            rating: 0.0,
            notes: "".to_string(),
            library_id: "".to_string(),
        }
    }
}

impl BookFull {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_db_and_api(book_db: Model, book_api_response: BooksApiResponse) -> Result<Self> {
        // check that at least the id is set or return error
        let book_id = if let Some(book_id) = book_api_response.get_id() {
            book_id
        } else {
            return Err(Error::MissingFields("missing fields: book_id".to_string()));
        };

        Ok(Self {
            id: book_db.id.to_string(),
            book_id,
            title: book_api_response.get_title(),
            authors: book_api_response.get_authors(),
            publisher: book_api_response.get_publisher(),
            published_date: book_api_response.get_publisher(),
            description: book_api_response.get_publisher(),
            isbn10: book_api_response.get_publisher(),
            isbn13: book_api_response.get_publisher(),
            page_count: book_api_response.get_page_count(),
            categories: book_api_response.get_categories(),
            language: book_api_response.get_publisher(),
            cover: book_api_response.get_publisher(),
            reading_status: book_db.reading_status.unwrap_or_default(),
            book_type: book_db.book_type.unwrap_or_default(),
            tags: book_db.tags.unwrap_or_default(),
            rating: book_db.rating.unwrap_or_default() as f32,
            notes: book_db.notes.unwrap_or_default(),
            library_id: book_db.library_id.unwrap_or_default(),
        })
    }
}
// endregion - BookFull

// region - UserBooks
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserBooks {
    user_id: String,
    books: Vec<BookFull>,
}

impl Default for UserBooks {
    fn default() -> Self {
        Self {
            user_id: "".to_string(),
            books: vec![],
        }
    }
}

impl UserBooks {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn from_user_id(user_id: String) -> Self {
        Self {
            user_id,
            books: vec![],
        }
    }

    pub fn get_user_id(&self) -> &str {
        &self.user_id
    }

    pub fn get_books(&self) -> &Vec<BookFull> {
        &self.books
    }

    pub fn set_user_id(&mut self, user_id: String) {
        self.user_id = user_id;
    }

    pub fn add_book(&mut self, book: BookFull) {
        self.books.push(book);
    }
}
// endregion - UserBooks
