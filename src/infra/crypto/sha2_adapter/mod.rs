use async_trait::async_trait;
use base64ct::{Base64, Encoding};
use sha2::{Digest, Sha256};

use crate::data::protocols::Encrypter;
use crate::GenericResult;

#[cfg(test)]
mod tests;

pub struct Sha2Adapter {
    encrypter: Box<dyn Encrypter>,
}

impl Sha2Adapter {
    pub fn new() -> Self {
        Self {
            encrypter: Box::new(StdEncrypter),
        }
    }

    /// Set the sha2 adapter's encrypter.
    pub fn set_encrypter(&mut self, encrypter: Box<dyn Encrypter>) {
        self.encrypter = encrypter;
    }
}

#[async_trait]
impl Encrypter for Sha2Adapter {
    async fn encrypt(&self, value: &str) -> GenericResult<String> {
        self.encrypter.encrypt(value).await
    }
}

struct StdEncrypter;

#[async_trait]
impl Encrypter for StdEncrypter {
    async fn encrypt(&self, value: &str) -> GenericResult<String> {
        let hash = Sha256::digest(value);
        let base64_hash = Base64::encode_string(&hash);

        Ok(base64_hash)
    }
}
