use async_trait::async_trait;

use crate::TError;

#[async_trait]
pub trait Encrypter: Send + Sync {
    async fn encrypt(&self, value: &str) -> TError<String>;
}
