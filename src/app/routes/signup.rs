use actix_web::post;
use actix_web::web::{self, ServiceConfig};
use serde::Serialize;

pub fn setup_signup_routes(cfg: &mut ServiceConfig) {
    cfg.service(create_account);
}

#[post("/signup")]
async fn create_account() -> web::Json<CreateAccountResponse> {
    let res = CreateAccountResponse {
        ok: String::from("ok"),
    };
    web::Json(res)
}

#[derive(Serialize)]
struct CreateAccountResponse {
    pub ok: String,
}

#[cfg(test)]
mod tests {
    use actix_web::dev::ServiceResponse;
    use actix_web::{dev::Service, test};
    use actix_web::{http, App};

    use crate::app;
    use crate::presentation::controllers::SignUpReqBodyBuilder;

    use super::create_account;

    #[actix_web::test]
    async fn returns_an_account_on_success() {
        let app = App::new()
            .configure(app::set_scope_api)
            .service(create_account);
        let app = test::init_service(app).await;

        let req_data = SignUpReqBodyBuilder::new()
            .set_name("Foo")
            .set_email("foo@gmail.com")
            .set_password("123")
            .set_password_confirmation("123")
            .build();

        let req = test::TestRequest::post()
            .uri("/signup")
            .set_json(req_data)
            .to_request();
        let res: ServiceResponse = app.call(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
