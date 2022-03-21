use mockall::automock;

use crate::GenericResult;

#[automock]
pub trait EmailValidator: Send + Sync {
    fn is_valid(&self, email: &str) -> GenericResult<bool>;
}
