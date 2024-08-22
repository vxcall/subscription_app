use actix_web::{middleware::from_fn, web};

use super::{handlers, middlewares};

// This in charge of every path in /auth path
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handlers::register)
            .service(handlers::auth_handlers::login)
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            .service(handlers::auth_handlers::logout),
    );
}
