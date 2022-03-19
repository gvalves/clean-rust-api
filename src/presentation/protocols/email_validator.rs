use crate::TError;

pub trait EmailValidator: Send + Sync {
    fn is_valid(&self, email: &str) -> TError<bool>;
}
