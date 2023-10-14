use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct OrderModel {
    pub id: i32,
    pub date: String,
    pub customer: String,
    pub price: i32,
    pub paid: bool,
    pub notes: String,
    pub items: sqlx::types::Json<Vec<ItemModel>>,
    pub username: Option<String>,
    pub deleted: Option<bool>,
    pub created: Option<DateTime<Utc>>,
    pub updated: Option<DateTime<Utc>>,
}
#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct ItemModel {
    //pub order_id: i32,
    pub product: String,
    pub quantity: i32,
    pub price: i32,
    // pub username: Option<String>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CountModel {
    pub count: i64,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
#[allow(non_snake_case)]
pub struct CustomerModel {
    pub customer: String,
}

#[derive(Deserialize, Debug, Default)]
pub struct Params {
    pub limit: Option<i32>,
    pub offset: Option<i32>,
    pub q: Option<String>,
    pub sortcol: Option<String>,
    pub desc: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ItemSchema {
    pub product: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct OrderSchema {
    pub id: i32,
    pub date: Option<String>,
    pub customer: Option<String>,
    pub price: Option<i32>,
    pub paid: Option<bool>,
    pub notes: Option<String>,
    pub items: Option<Vec<ItemSchema>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateOrderSchema {
    pub date: String,
    pub customer: String,
    pub price: i32,
    pub paid: bool,
    pub notes: String,
    pub items: Vec<ItemSchema>,
}
