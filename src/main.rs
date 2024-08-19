use actix_web::{middleware::Logger, App, HttpServer};
use sea_orm::Database;

mod routes;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    dotenv::dotenv().ok();
    env_logger::init();

    let address = (utils::constants::ADDRESS).clone();
    let port = (utils::constants::PORT).clone();
    let database_url = (utils::constants::DATABASE_URL).clone();

    // let db = Database::connect(database_url).await.unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
