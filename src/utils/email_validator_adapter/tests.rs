use crate::presentation::protocols::email_validator::EmailValidator;

use super::EmailValidatorAdapter;

#[test]
fn returns_false_if_validator_returns_false() {
    let sut = EmailValidatorAdapter;
    let is_valid = sut.is_valid("invalid_email@mail.com").unwrap();
    assert!(!is_valid);
}
