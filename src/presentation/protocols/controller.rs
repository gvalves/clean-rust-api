use async_trait::async_trait;

use crate::presentation::http::{HttpRequest, HttpResponse};

#[async_trait]
pub trait ControllerProtocol<ReqBody: Send, ResBody: Send> {
    async fn handle(&self, req: HttpRequest<ReqBody>) -> HttpResponse<ResBody>;
}
