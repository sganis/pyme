use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct Params {
    pub page: Option<usize>,
    pub limit: Option<usize>,
    pub q: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemSchema {
    pub date: String,
    pub customer: String,
    pub product: String,
    pub quantity: i32,
    pub price: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemSchema {
    pub date: Option<String>,
    pub customer: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<i32>,
}
