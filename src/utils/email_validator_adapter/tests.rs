use mockall::predicate;
use mockall_double::double;

use crate::presentation::protocols::EmailValidator;

#[double]
use super::EmailValidatorAdapter;

fn make_sut() -> EmailValidatorAdapter {
    EmailValidatorAdapter::default()
}

#[test]
fn returns_false_if_validator_returns_false() {
    let mut sut = make_sut();
    sut.expect_is_valid().returning(|_| Ok(false));

    let is_valid = sut.is_valid("invalid_email@mail.com").unwrap();
    assert!(!is_valid);
}

#[test]
fn returns_true_if_validator_returns_true() {
    let mut sut = make_sut();
    sut.expect_is_valid().returning(|_| Ok(true));

    let is_valid = sut.is_valid("valid_email@mail.com").unwrap();
    assert!(is_valid);
}

#[test]
fn calls_validator_with_correct_email() {
    let mut sut = make_sut();

    sut.expect_is_valid()
        .with(predicate::eq("any_email@mail.com"))
        .returning(|_| Ok(true));

    sut.is_valid("any_email@mail.com").unwrap();
}

#[test]
fn calls_validator_when_validate() {
    #[double]
    use crate::presentation::protocols::EmailValidator;

    use super::EmailValidatorAdapter;

    let mut validator = EmailValidator::default();
    validator.expect_is_valid().once().returning(|_| Ok(true));

    let mut sut = EmailValidatorAdapter::new();
    sut.set_validator(Box::new(validator));

    sut.is_valid("valid_email@mail.com").unwrap();
}
