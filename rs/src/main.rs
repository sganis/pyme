mod handler;
mod model;
mod route;
mod schema;
mod auth;


use std::sync::Arc;
// use std::net::SocketAddr;
// use dotenv::dotenv;
use route::create_router;
use tower_http::cors::CorsLayer;
use sqlx::{postgres::PgPoolOptions, Pool, PgPool, Postgres};
// // use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use axum::Router;
use axum::http::{
     header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE},
     Method,
};

pub struct AppState {
    db: Pool<Postgres>,
}

use axum::{routing::get, Router, Error};

// async fn hello_world() -> &'static str {
//     "Hello, world!"
// }

// #[shuttle_runtime::main]
// async fn axum() -> shuttle_axum::ShuttleAxum {
//     let router = Router::new().route("/", get(hello_world));

//     Ok(router.into())
// }


#[shuttle_runtime::main]
pub async fn axum (
//#[shuttle_shared_db::Postgres] postgres: PgPool,
#[shuttle_secrets::Secrets] secrets: shuttle_secrets::SecretStore
) -> shuttle_axum::ShuttleAxum {

    // get secret defined in `Secrets.toml` file.
    let database_url = if let Some(database_url) = secrets.get("DATABASE_URL") {
        database_url
    } else {
        "".to_string()
    };
    println!("database={}",database_url);
    let jwt_secret = if let Some(jwt_secret) = secrets.get("JWT_SECRET") {
        std::env::set_var("JWT_SECRET", jwt_secret.clone());
        jwt_secret
    } else {
        "".to_string()
    };
    println!("jwt secret={}",jwt_secret);

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
    // sqlx::migrate!()
    //     .run(&postgres)
    //     .await
    //     .expect("Migrations failed :(");

    let cors = CorsLayer::new()
        .allow_origin([
            "http://localhost:8000".parse().unwrap(),
            "http://localhost:5173".parse().unwrap(),
        ])
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_credentials(true)
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

    let router = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

    println!("Server started successfully at 127.0.0.1:8000");


    //let router = Router::new().route("/", get(hello_world));
    //let router = create_router(postgres);

    Ok(router.into())
}


// // #[tokio::main]
// #[shuttle_runtime::main]
// async fn axum() -> shuttle_axum::ShuttleAxum {
//     // tracing_subscriber::registry()
//     //     .with(
//     //         tracing_subscriber::EnvFilter::try_from_default_env()
//     //             .unwrap_or_else(|_| "pyme=debug".into()),
//     //     )
//     //     .with(tracing_subscriber::fmt::layer())
//     //     .init();

//     dotenv().ok();

//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set");
//     let pool = match PgPoolOptions::new()
//         .max_connections(10)
//         .connect(&database_url)
//         .await {
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
//             "http://127.0.0.0:5173".parse().unwrap(),
//         ])
//         .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
//         .allow_credentials(true)
//         .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE]);

//     let router = create_router(Arc::new(AppState { db: pool.clone() })).layer(cors);

//     println!("Server started successfully at 127.0.0.1:8000");

//     // let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
//     // let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
//     // axum::serve(listener, app).await.unwrap();
//     Ok(router.into())
// }



