// use std::net::{Ipv6Addr, SocketAddr};
use tokio::net::TcpListener;
use crate::router::create_router;
// use crate::error::AppError;
use crate::app_state::{AppState, SharedState};
use crate::config::{AppConfig, connect_db};


// pub fn get_env_vars<T>(key: String) -> Result<T, AppError>
// where
//     T: std::str::FromStr,
//     T::Err: std::fmt::Display,
// {
//     let value = std::env::var(&key).map_err(|_| AppError::MissingEnvironmentVarible(key))?;
//     value
//         .parse::<T>()
//         .map_err(|err| AppError::ParsingError(err.to_string()))

    
// }

// pub async fn serve() {
//     let port: u16 = get_env_vars::<u16>("PORT".to_string()).unwrap_or(8080);
//     let addr = SocketAddr::from((Ipv6Addr::UNSPECIFIED, port));
//     let listener = TcpListener::bind(addr)
//         .await
//         .expect("Failed to bind address");

//     tracing::info!("Server is listening on: {}", listener.local_addr().unwrap());

//     let app = create_app();
//     axum::serve(listener, app).await.unwrap();
// }


pub async fn create_app() {
    // Get the environment variables
    let app_config = AppConfig::from_env().expect("Failed to load configuration from environment");

    // Get the db pool from the database url returned by AppConfig.database_url
    let db_pool = connect_db(&app_config.database_url)
        .await
        .expect("Failed to connect to the database");

    let app_state = SharedState::new(AppState::new(db_pool));

    let app = create_router(app_state);

    let server_address = format!("127.0.0.1:{}", app_config.server_port);

    let listener = TcpListener::bind(&server_address).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}