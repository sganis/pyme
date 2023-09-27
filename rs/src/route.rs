use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use crate::{
    handler::{
        token, protected,
        get_items, get_item, create_item, update_item, delete_item 
    },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
    .route("/token", post(token))
    .route("/protected", get(protected))
    .route("/pyme/", get(get_items).post(create_item))
    .route("/pyme/:id", get(get_item).put(update_item).delete(delete_item))
    .with_state(app_state)
}
