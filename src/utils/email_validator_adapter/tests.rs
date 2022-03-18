use std::error;

use crate::presentation::protocols::email_validator::EmailValidator;

struct EmailValidatorAdapterMock {
    strategy: Box<dyn EmailValidator>,
}

impl EmailValidatorAdapterMock {
    fn new() -> Self {
        Self {
            strategy: Box::new(EmailValidatorAlwaysTrueStrategy),
        }
    }

    /// Set the email validator adapter mock's strategy.
    fn set_strategy(&mut self, strategy: Box<dyn EmailValidator>) {
        self.strategy = strategy;
    }
}

impl EmailValidator for EmailValidatorAdapterMock {
    fn is_valid(&self, email: &str) -> Result<bool, Box<dyn error::Error>> {
        self.strategy.is_valid(email)
    }
}

struct EmailValidatorAlwaysTrueStrategy;

impl EmailValidator for EmailValidatorAlwaysTrueStrategy {
    fn is_valid(&self, _: &str) -> Result<bool, Box<dyn error::Error>> {
        Ok(true)
    }
}

struct EmailValidatorAlwaysFalseStrategy;

impl EmailValidator for EmailValidatorAlwaysFalseStrategy {
    fn is_valid(&self, _: &str) -> Result<bool, Box<dyn error::Error>> {
        Ok(false)
    }
}

fn make_sut() -> EmailValidatorAdapterMock {
    EmailValidatorAdapterMock::new()
}

#[test]
fn returns_false_if_validator_returns_false() {
    let mut sut = make_sut();
    sut.set_strategy(Box::new(EmailValidatorAlwaysFalseStrategy));

    let is_valid = sut.is_valid("invalid_email@mail.com").unwrap();
    assert!(!is_valid);
}

#[test]
fn returns_true_if_validator_returns_true() {
    let sut = make_sut();
    let is_valid = sut.is_valid("valid_email@mail.com").unwrap();
    assert!(is_valid);
}

#[test]
fn calls_validator_with_correct_email() {
    struct EmailValidatorAssertEmailStrategy;

    impl EmailValidator for EmailValidatorAssertEmailStrategy {
        fn is_valid(&self, email: &str) -> Result<bool, Box<dyn error::Error>> {
            assert_eq!(email, "any_email@mail.com");
            Ok(true)
        }
    }

    let mut sut = make_sut();
    sut.set_strategy(Box::new(EmailValidatorAssertEmailStrategy));
    sut.is_valid("any_email@mail.com").unwrap();
}
