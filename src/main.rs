use actix_web::{App, HttpServer};
use clean_rust_api::setup_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(setup_app))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}
