use tokio;

use crate::{
    presentation::{http::HttpRequest, protocols::controller::ControllerProtocol},
    ErrorMsg,
};

use super::{SignUpBodyRes, SignUpController};

#[tokio::test]
pub async fn returns_400_if_req_body_is_none() {
    let sut = SignUpController::new();
    let req = HttpRequest::new(None);
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpBodyRes::Err(ErrorMsg::new("missing body"))
    );
}
