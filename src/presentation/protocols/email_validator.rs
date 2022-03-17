pub trait EmailValidator: Sync {
    fn is_valid(&self, email: &str) -> bool;
}
