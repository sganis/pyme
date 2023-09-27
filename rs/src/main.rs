mod handler;
mod model;
mod route;
mod schema;
mod auth;


use std::sync::Arc;
use std::net::SocketAddr;
use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};

pub struct AppState {
    db: Pool<Postgres>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "pyme=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await {
        Ok(pool) => {
            println!("Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:8000".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
            "http://127.0.0.0:5173".parse().unwrap(),
            "https://pyme-sganis.vercel.app".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let app = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("Server started successfully at 127.0.0.1:8000");

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}



