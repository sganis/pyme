use crate::{
    handler::{
        create_item, delete_item, get_customers, get_item, get_items, get_products,
        get_stat_customer, get_stat_product, get_stat_quarter, get_stat_year, 
        password, token,
        update_item,
        wakeup,
    },
    test::{fibi, fibr, ping},
    AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;
use tower_http::services::{ServeDir, ServeFile};

pub fn create_router(state: Arc<AppState>) -> Router {
    let spa = ServeDir::new("frontend/dist")
        .not_found_service(ServeFile::new("frontend/dist/index.html"));
    Router::new()
        .route("/ping", get(ping))
        .route("/wakeup", get(wakeup))
        .route("/fibr/:n", get(fibr))
        .route("/fibi/:n", get(fibi))
        .route("/token", post(token))
        .route("/password", post(password))
        .route("/pyme/", get(get_items).post(create_item).put(update_item))
        .route("/pyme/customers/", get(get_customers))
        .route("/pyme/products/", get(get_products))
        .route("/pyme/stat/customers/", get(get_stat_customer))
        .route("/pyme/stat/products/", get(get_stat_product))
        .route("/pyme/stat/years/", get(get_stat_year))
        .route("/pyme/stat/quarters/", get(get_stat_quarter))
        .route("/pyme/:id", get(get_item).delete(delete_item))
        .nest_service("/", spa.clone())
        .fallback_service(spa)
        .with_state(state)
}
