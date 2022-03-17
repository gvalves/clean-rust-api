use std::error;

pub trait EmailValidator: Sync {
    fn is_valid(&self, email: &str) -> Result<bool, Box<dyn error::Error>>;
}
