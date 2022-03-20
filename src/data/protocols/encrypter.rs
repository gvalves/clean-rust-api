use async_trait::async_trait;
use mockall::automock;

use crate::TError;

#[automock]
#[async_trait]
pub trait Encrypter: Send + Sync {
    async fn encrypt(&self, value: &str) -> TError<String>;
}
