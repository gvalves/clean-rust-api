use mockall::predicate;
use mockall_double::double;

#[double]
use crate::data::protocols::Encrypter as MockEncrypter;

use crate::data::protocols::Encrypter;
use crate::ErrorMsg;

use super::Sha2Adapter;

macro_rules! encrypter_encrypt_default {
    () => {
        |_| Ok(String::from("hashed_value"))
    };
}

fn make_sut() -> Sha2Adapter {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .returning(encrypter_encrypt_default!());

    let mut sut = Sha2Adapter::new();
    sut.set_encrypter(encrypter);

    sut
}

fn make_encrypter() -> Box<MockEncrypter> {
    Box::new(MockEncrypter::default())
}

#[tokio::test]
async fn calls_encrypter() {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .once()
        .returning(encrypter_encrypt_default!());

    let mut sut = make_sut();
    sut.set_encrypter(encrypter);

    match sut.encrypt("any_value").await {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[tokio::test]
async fn calls_encrypter_with_correct_data() {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .with(predicate::eq("any_value"))
        .returning(encrypter_encrypt_default!());

    let mut sut = make_sut();
    sut.set_encrypter(encrypter);

    match sut.encrypt("any_value").await {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[tokio::test]
async fn returns_err_if_encrypter_returns_err() {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .returning(|_| ErrorMsg::default().into());

    let mut sut = make_sut();
    sut.set_encrypter(encrypter);

    let result = sut.encrypt("any_value").await;

    assert_eq!(
        result.unwrap_err().to_string(),
        ErrorMsg::default().to_string()
    );
}

#[tokio::test]
async fn returns_a_hash_on_success() {
    let sut = make_sut();
    let result = sut.encrypt("any_value").await;

    assert_eq!(result.unwrap(), "hashed_value");
}
