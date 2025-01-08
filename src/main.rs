mod db;
mod handlers;
mod schema;
mod actions;
mod models;

use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use handlers::{get_users, create_user};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    // Initialize the database pool
    let pool = db::init_db_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone())) // Share DB pool across workers
            .service(get_users)
            .service(create_user)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}