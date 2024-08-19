use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::Database;
use utils::app_status::AppState;

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

    println!("DATAAAAAAAAAAAAAAAAAAA: {database_url}");

    let db = Database::connect(database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))?
    .run()
    .await
}
