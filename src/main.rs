mod auth;
mod handler;
mod schema;
mod route;
mod test;

use axum::http::{
    header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
    Method,
};
// use axum::{
//     http::HeaderValue,
//     response::{Html, IntoResponse},
//     routing::get,
//     Json, Router,
// };
// use dotenvy::dotenv;
// use std::net::SocketAddr;

use route::create_router;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub struct AppState {
    db: Pool<Postgres>,
}

#[shuttle_runtime::main]
pub async fn axum(
    #[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore,
) -> shuttle_axum::ShuttleAxum {
    std::env::set_var(
        "JWT_SECRET",
        secrets
            .get("JWT_SECRET")
            .expect("JWT_SECRET secret is missing"),
    );
    let database_url = secrets
        .get("DATABASE_URL")
        .expect("DATABASE_URL secret is missing");
    std::env::set_var("DATABASE_URL", database_url.clone());

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
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
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router =
        create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);
    println!("Server started successfully");

    Ok(router.into())
}

// #[tokio::main]
// async fn main() {
//     dotenv().expect(".env not found");
//     let database_url = std::env::var("DATABASE_URL").unwrap();
//     let pool = match PgPoolOptions::new()
//         .max_connections(10)
//         .connect(&database_url)
//         .await
//     {
//         Ok(pool) => {
//             println!("Connection to the database is successful!");
//             pool
//         }
//         Err(err) => {
//             println!("Failed to connect to the database: {:?}", err);
//             std::process::exit(1);
//         }
//     };

//     let cors = CorsLayer::new()
//         .allow_origin([
//             "http://localhost:8000".parse().unwrap(),
//             "http://localhost:5173".parse().unwrap(),
//         ])
//         .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
//         .allow_credentials(true)
//         .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

//     let router = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);
//     println!("Server started successfully");

//     let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//     axum::Server::bind(&addr)
//         .serve(router.into_make_service())
//         .await
//         .unwrap();
// }
