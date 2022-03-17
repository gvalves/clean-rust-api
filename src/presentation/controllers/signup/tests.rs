use tokio;

use crate::presentation::protocols::controller::ControllerProtocol;
use crate::presentation::{http::HttpRequest, protocols::email_validator::EmailValidator};
use crate::ErrorMsg;

use super::{SignUpController, SignUpReqBodyBuilder, SignUpResBody};

fn make_sut() -> SignUpController {
    let email_validator = make_email_validator();
    SignUpController::new(email_validator)
}

fn make_email_validator() -> Box<dyn EmailValidator> {
    make_email_validator_strategy(|_| true)
}

fn make_email_validator_strategy<T>(strategy: T) -> Box<dyn EmailValidator>
where
    T: Fn(&str) -> bool + Sync + 'static,
{
    struct EmailValidatorStub<T>
    where
        T: Fn(&str) -> bool + Sync,
    {
        strategy: T,
    }

    impl<T> EmailValidator for EmailValidatorStub<T>
    where
        T: Fn(&str) -> bool + Sync,
    {
        fn is_valid(&self, email: &str) -> bool {
            let strategy = &self.strategy;
            strategy(email)
        }
    }

    Box::new(EmailValidatorStub { strategy })
}

#[tokio::test]
pub async fn returns_400_if_req_body_is_none() {
    let sut = make_sut();
    let req = HttpRequest::new(None);
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing body"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_name_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'name'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_email_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'email'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_password_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'password'"))
    );
}
#[tokio::test]
pub async fn returns_400_if_no_password_confirmation_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'password_confirmation'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_password_confirmation_fails() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("invalid_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("invalid param 'password_confirmation'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_invalid_email_is_provided() {
    let mut sut = make_sut();
    sut.set_email_validator(make_email_validator_strategy(|_| false));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("invalid_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("invalid param 'email'"))
    );
}
