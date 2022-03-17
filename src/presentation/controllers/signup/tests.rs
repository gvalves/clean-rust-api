use tokio;

use crate::presentation::http::HttpRequest;
use crate::presentation::protocols::controller::ControllerProtocol;
use crate::ErrorMsg;

use super::{SignUpController, SignUpReqBodyBuilder, SignUpResBody};

#[tokio::test]
pub async fn returns_400_if_req_body_is_none() {
    let sut = SignUpController::new();
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
    let sut = SignUpController::new();
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
