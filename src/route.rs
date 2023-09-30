use std::sync::Arc;
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::services::{ServeDir, ServeFile};
use crate::{
    handler::{
        token,
        get_items, get_item, create_item, update_item, delete_item,
        get_customers,
        get_stat_customer, get_stat_product, get_stat_year, get_stat_quarter
    },
    AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let spa = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));
    Router::new()
    .route("/token", post(token))
    .route("/pyme/", get(get_items)
        .post(create_item))
    .route("/pyme/customer/", get(get_customers))
    .route("/pyme/stat/customer", get(get_stat_customer))
    .route("/pyme/stat/product", get(get_stat_product))
    .route("/pyme/stat/year", get(get_stat_year))
    .route("/pyme/stat/quarter", get(get_stat_quarter))
    .route("/pyme/:id", get(get_item)
        .put(update_item)
        .delete(delete_item))
    .nest_service("/", spa.clone())
    .fallback_service(spa)
    .with_state(state)
}
