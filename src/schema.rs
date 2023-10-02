use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Default)]
pub struct Params {
    pub page: Option<i32>,
    pub limit: Option<i32>,
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

