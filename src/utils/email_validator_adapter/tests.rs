use crate::presentation::protocols::email_validator::EmailValidator;
use crate::TError;

use super::EmailValidatorAdapter;

struct EmailValidatorAdapterMock {
    strategy: Box<dyn EmailValidator>,
    sut: EmailValidatorAdapter,
}

impl EmailValidatorAdapterMock {
    fn new() -> Self {
        let strategy = Self::make_strategy(|_| Ok(true));
        Self {
            strategy,
            sut: EmailValidatorAdapter,
        }
    }

    fn make_strategy<T>(callback: T) -> Box<dyn EmailValidator>
    where
        T: Fn(&str) -> TError<bool> + Send + Sync + 'static,
    {
        struct EmailValidatorAdapterMockStrategy<T>
        where
            T: Fn(&str) -> TError<bool> + Send + Sync,
        {
            callback: T,
        }

        impl<T> EmailValidator for EmailValidatorAdapterMockStrategy<T>
        where
            T: Fn(&str) -> TError<bool> + Send + Sync,
        {
            fn is_valid(&self, email: &str) -> TError<bool> {
                let callback = &self.callback;
                callback(email)
            }
        }

        Box::new(EmailValidatorAdapterMockStrategy { callback })
    }

    /// Set the email validator adapter mock's strategy.
    fn set_strategy(&mut self, strategy: Box<dyn EmailValidator>) {
        self.strategy = strategy;
    }
}

impl EmailValidator for EmailValidatorAdapterMock {
    fn is_valid(&self, email: &str) -> TError<bool> {
        let res = self.strategy.is_valid(email);
        self.sut.is_valid(email).unwrap_or_default();
        res
    }
}

fn make_sut() -> EmailValidatorAdapterMock {
    EmailValidatorAdapterMock::new()
}

#[test]
fn returns_false_if_validator_returns_false() {
    let mut sut = make_sut();
    sut.set_strategy(EmailValidatorAdapterMock::make_strategy(|_| Ok(false)));

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
    let mut sut = make_sut();
    sut.set_strategy(EmailValidatorAdapterMock::make_strategy(|email| {
        assert_eq!(email, "any_email@mail.com");
        Ok(true)
    }));
    sut.is_valid("any_email@mail.com").unwrap();
}
