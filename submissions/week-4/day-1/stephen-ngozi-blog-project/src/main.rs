use std::{
    any::Any,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
};

use axum::{Router, routing::get};
// use chrono::naive::serde;
use thiserror::Error;
use tokio::net::TcpListener;
use tracing_subscriber::{EnvFilter, fmt};

fn trace_setup() {
    let env_filter: EnvFilter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt()
        .with_target(false)
        .pretty()
        .with_env_filter(env_filter)
        .init();

    
}

#[derive(Debug, Error)]
enum AppError {
    #[error("Resource not found")]
    NotFound,
    #[error("Internal Server Error: {0}")]
    InternalServerError(String),
    #[error("Invalid Input, cannot be processed: {field} - {message}")]
    UnProcessableEntity { field: String, message: String },
    #[error("Environement Variable is missing: {0}")]
    MissingEnvironmentVarible(String),
    #[error("Failed to Parse: {0}")]
    ParsingError(String),
}

async fn health() -> &'static str {
    "Hello, World!, app is working fine"
}

// async fn connect_db() -> Result<sqlx::PgPool, AppError> {
//     let db_url = get_env_vars::<String>("DATABASE_URL".to_string()).unwrap();

//     let pg_pool: sqlx::Pool<sqlx::Postgres> = sqlx::postgres::PgPoolOptions::new()
//         .connect(&db_url)
//         .await
//         .map_err(|_| AppError::NotFound)?;

//     sqlx::migrate!("./migrations")
//         .run(&pg_pool)
//         .await
//         .map_err(|err| AppError::InternalServerError(err.to_string()))?;

//     // if let Ok(db_url) = db_url {
//     //     tracing::info!("Connecting to database at: {}", db_url);
//     // } else {
//     //     tracing::error!("Database URL is not set in environment variables");
//     // }

//     Ok(pg_pool)
// }

#[derive(Debug)]
pub struct Post {
    id: uuid::Uuid,
    title: String,
    content: String,
    author: uuid::Uuid,
    created_at: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug)]
pub struct Author {
    id: uuid::Uuid,
    name: String,
    email: String,
    created_at: chrono::DateTime<chrono::Utc>,
}

fn create_app() -> Router {
    Router::new().route("/health", get(health))
}

fn get_env_vars<T>(key: String) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    let value = std::env::var(&key).map_err(|_| AppError::MissingEnvironmentVarible(key))?;
    value
        .parse::<T>()
        .map_err(|err| AppError::ParsingError(err.to_string()))

    // let value = std::env::var(key)
    //     .map_err(|err| format!("Failed to read environment variable: {}", err))?;
    // value.parse::<T>().map_err(|_| "".to_string())
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    trace_setup();
    let app: Router = create_app();
    let listening_port: u16 = 8080;

    let port: u16 = get_env_vars::<u16>("PORT".to_string()).unwrap_or(listening_port);
    tracing::info!("Starting server on port: {}", port);
    let listening_address: SocketAddr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));

    let binder: TcpListener = TcpListener::bind(listening_address)
        .await
        .expect("Failed to bind address");

    // .expect(String::from(AppError::NotFound).as_str());

    // println!("Server is listening on {}", binder.local_addr().unwrap());
    tracing::info!("Server is listening on: {}", binder.local_addr().unwrap());
    axum::serve(binder, app).await.unwrap();
}