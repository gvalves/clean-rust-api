use actix_web::{get, middleware, App, HttpServer, Responder};

#[get("/")]
async fn hello() -> impl Responder {
    String::from("Hello World!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello))
        .bind(("127.0.0.1", 8000))?
        .run()
        .await
}

#[cfg(test)]
mod tests {
    #[actix_web::test]
    async fn test_hello() {}
}
