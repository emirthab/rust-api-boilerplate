#[macro_use]
extern crate diesel;

mod api;
mod models;
mod errors;
mod jwt;
mod config;

use std::borrow::Borrow;

use actix_web::{web, App, HttpResponse, HttpServer, Result, middleware};
use config::Config;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};

// We define a custom type for connection pool to use later.
pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbError = Box<dyn std::error::Error + Send + Sync>;

pub struct AppState {
    db: DbPool,
    env: Config,
}

async fn default_route() -> Result<HttpResponse> {
    let response = errors::HttpError {
        message: "Resource not found".to_string(),
        code: 404
    };
    Ok(HttpResponse::NotFound().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    let config = Config::init();
    
    let manager = ConnectionManager::<PgConnection>::new(config.database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState{
                db: pool.clone(),
                env: config.clone()
            }))
            .configure(api::config)
            .default_service(web::route().to(default_route))
            .wrap(middleware::Logger::default())
    })
    .bind((
        std::env::var("APP_HOST").expect("APP_HOST must be set"),
        std::env::var("APP_PORT")
            .expect("APP_PORT must be set")
            .parse::<u16>()
            .unwrap(),
    ))?
    .run()
    .await
}
