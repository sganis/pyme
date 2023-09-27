use std::sync::Arc;

use axum::{
    routing::{get},
    Router,
};

use crate::{
    //auth::token,
    handler::{get_items, get_item, create_item, update_item, delete_item },
    AppState,
};

pub fn create_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        //.route("/token", post(token))
        .route("/pyme/", get(get_items).post(create_item))
        .route("/pyme/:id", get(get_item).put(update_item).delete(delete_item))
        .with_state(app_state)
}
