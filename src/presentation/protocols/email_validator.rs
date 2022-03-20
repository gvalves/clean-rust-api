use mockall::automock;

use crate::TError;

#[automock]
pub trait EmailValidator: Send + Sync {
    fn is_valid(&self, email: &str) -> TError<bool>;
}
