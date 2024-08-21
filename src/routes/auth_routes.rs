use actix_web::web;

use super::handlers;

// This in charge of every path in /auth path
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handlers::register)
            .service(handlers::auth_handlers::login),
    );
}
