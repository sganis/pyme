use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct FilterOptions {
    pub page: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Deserialize, Debug)]
pub struct ParamOptions {
    pub id: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateItemSchema {
    pub date: String,
    pub customer: String,
    pub product: String,
    pub quantity: i32,
    pub price: i32,
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub category: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UpdateItemSchema {
    pub date: Option<String>,
    pub customer: Option<String>,
    pub product: Option<String>,
    pub quantity: Option<i32>,
    pub price: Option<i32>,
}
