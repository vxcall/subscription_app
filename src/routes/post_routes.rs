use actix_web::web;
use actix_web_lab::middleware::from_fn;

use super::{handlers, middlewares};

// This in charge of every path in /user path
pub fn config(config: &mut web::ServiceConfig) {
    config
        .service(
            web::scope("/post").wrap(from_fn(middlewares::auth_middleware::check_auth_middleware)),
        )
        .service(web::scope("/post"));
}
