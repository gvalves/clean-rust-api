use mockall_double::double;

use crate::data::protocols::encrypter::Encrypter;
#[double]
use crate::data::protocols::encrypter::Encrypter as MockEncrypter;

use super::Sha2Adapter;

macro_rules! encrypter_encrypt_default {
    () => {
        |_| Ok(String::from("any_hash"))
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
