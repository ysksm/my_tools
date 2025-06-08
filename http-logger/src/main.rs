use std::io;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
    })
    .bind(("127.0.0.1", 8080))?
    .workers(2)
    .run()
    .await
}