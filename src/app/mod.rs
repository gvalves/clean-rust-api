use actix_web::web::{self, ServiceConfig};

use self::routes::signup::setup_signup_routes;

pub mod routes;

pub fn setup_app(cfg: &mut ServiceConfig) {
    set_scope_api(cfg);
    cfg.configure(setup_signup_routes);
}

pub(crate) fn set_scope_api(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/api"));
}
