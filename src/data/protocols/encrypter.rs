use async_trait::async_trait;
use mockall::automock;

use crate::GenericResult;

#[automock]
#[async_trait]
pub trait Encrypter: Send + Sync {
    async fn encrypt(&self, value: &str) -> GenericResult<String>;
}
