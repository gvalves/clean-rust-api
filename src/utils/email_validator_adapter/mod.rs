use validator::Validate;

use crate::presentation::protocols::email_validator::EmailValidator;

#[cfg(test)]
pub mod tests;

pub struct EmailValidatorAdapter;

impl EmailValidator for EmailValidatorAdapter {
    fn is_valid(&self, email: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let email = String::from(email);
        let data = DataToValidate { email };

        Ok(data.validate().is_ok())
    }
}

#[derive(Validate)]
struct DataToValidate {
    #[validate(email)]
    email: String,
}
