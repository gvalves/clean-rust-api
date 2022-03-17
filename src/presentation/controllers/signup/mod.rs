use async_trait::async_trait;

use crate::{
    presentation::{
        http::{HttpRequest, HttpResponse},
        protocols::controller::ControllerProtocol,
    },
    ErrorMsg,
};

#[cfg(test)]
pub mod tests;

pub struct SignUpController;

impl SignUpController {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl ControllerProtocol<SignUpBodyReq, SignUpBodyRes> for SignUpController {
    async fn handle(&self, req: HttpRequest<SignUpBodyReq>) -> HttpResponse<SignUpBodyRes> {
        HttpResponse::new(400, SignUpBodyRes::Err(ErrorMsg::new("missing body")))
    }
}

#[derive(Debug, PartialEq, PartialOrd)]
struct SignUpBodyReq {}

#[derive(Debug, PartialEq, PartialOrd)]
enum SignUpBodyRes {
    Account(u32),
    Err(ErrorMsg),
}
