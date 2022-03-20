use mockall::automock;
use validator::Validate;

use crate::presentation::protocols::email_validator::EmailValidator;
use crate::TError;

#[cfg(test)]
pub mod tests;

pub struct EmailValidatorAdapter {
    validator: Box<dyn EmailValidator>,
}

impl EmailValidatorAdapter {
    pub fn new() -> Self {
        Self {
            validator: Box::new(StdEmailValidator::default()),
        }
    }

    /// Set the email validator adapter's validator.
    pub fn set_validator(&mut self, validator: Box<dyn EmailValidator>) {
        self.validator = validator;
    }
}

impl Default for EmailValidatorAdapter {
    fn default() -> Self {
        Self {
            validator: Box::new(StdEmailValidator::default()),
        }
    }
}

#[automock]
impl EmailValidator for EmailValidatorAdapter {
    fn is_valid(&self, email: &str) -> TError<bool> {
        self.validator.is_valid(email)
    }
}

#[derive(Validate)]
struct StdEmailValidator {
    #[validate(email)]
    email: String,
}

impl Default for StdEmailValidator {
    fn default() -> Self {
        Self {
            email: Default::default(),
        }
    }
}

impl EmailValidator for StdEmailValidator {
    fn is_valid(&self, email: &str) -> TError<bool> {
        let email = String::from(email);
        let data = StdEmailValidator { email };
        Ok(data.validate().is_ok())
    }
}
