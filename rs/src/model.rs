use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ItemModel {
    pub id: i32,
    pub date: String,
    pub customer: String,
    pub product: String,
    pub quantity: i32,
    pub price: i32,
    pub deleted: Option<bool>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}
pub struct ProductModel {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub deleted: Option<bool>,
    pub created: Option<chrono::DateTime<chrono::Utc>>,
    pub updated: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CustomerModel {
    pub customer: String,
}

