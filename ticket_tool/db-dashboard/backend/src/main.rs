mod db;
mod error;
mod handlers;
mod models;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    
    let db_path = std::env::var("DB_PATH").unwrap_or_else(|_| "dashboard.db".to_string());
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let host = std::env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    
    log::info!("Starting server on {}:{}", host, port);
    log::info!("Using database: {}", db_path);
    
    let database = Arc::new(
        db::Database::new(&db_path)
            .expect("Failed to initialize database")
    );
    
    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .max_age(3600);
        
        App::new()
            .app_data(web::Data::new(database.clone()))
            .wrap(cors)
            .wrap(Logger::default())
            .route("/api/health", web::get().to(handlers::health_check))
            .route("/api/schema", web::get().to(handlers::get_schema))
            .route("/api/data", web::get().to(handlers::get_table_data))
            .route("/api/query", web::post().to(handlers::execute_query))
    })
    .bind((host, port.parse::<u16>().expect("Invalid port")))?
    .run()
    .await
}
