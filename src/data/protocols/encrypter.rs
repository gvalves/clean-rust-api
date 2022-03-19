use std::error;

use async_trait::async_trait;

#[async_trait]
pub trait Encrypter: Send + Sync {
    async fn encrypt(&self, value: &str) -> Result<String, Box<dyn error::Error>>;
}
