use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod errors;
mod models;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = sqlx::MySqlPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/register").route(web::post().to(controllers::auth::register)))
            .service(web::resource("/login").route(web::post().to(controllers::auth::login)))
            .service(web::resource("/profile").route(web::get().to(controllers::auth::profile)))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
