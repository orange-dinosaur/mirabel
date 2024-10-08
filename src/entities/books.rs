//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0

use sea_orm::entity::prelude::*;
use serde::Serialize;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize)]
#[sea_orm(table_name = "books")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub created_at: Option<DateTime>,
    pub updated_at: Option<DateTime>,
    #[sea_orm(column_type = "Text")]
    pub book_id: String,
    #[sea_orm(column_type = "Text")]
    pub user_id: String,
    #[sea_orm(column_type = "Text", nullable)]
    pub reading_status: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub book_type: Option<String>,
    pub tags: Option<Vec<String>>,
    #[sea_orm(column_type = "Double", nullable)]
    pub rating: Option<f64>,
    #[sea_orm(column_type = "Text", nullable)]
    pub notes: Option<String>,
    #[sea_orm(column_type = "Text", nullable)]
    pub library_id: Option<String>,
    pub reading_start_date: Option<Date>,
    pub reading_end_date: Option<Date>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
