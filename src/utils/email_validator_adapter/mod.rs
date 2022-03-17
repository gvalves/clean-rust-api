use crate::presentation::protocols::email_validator::EmailValidator;

#[cfg(test)]
pub mod tests;

pub struct EmailValidatorAdapter;

impl EmailValidator for EmailValidatorAdapter {
    fn is_valid(&self, email: &str) -> Result<bool, Box<dyn std::error::Error>> {
        Ok(false)
    }
}
