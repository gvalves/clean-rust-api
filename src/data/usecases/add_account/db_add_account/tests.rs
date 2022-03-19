use async_trait::async_trait;
use tokio;

use crate::{
    data::protocols::encrypter::Encrypter,
    domain::usecases::add_account::{AddAccount, AddAccountDto},
    ErrorMsg, TError,
};

use super::DbAddAccount;

fn make_sut() -> DbAddAccount {
    let encrypter = make_encrypter(|_| Ok(String::from("hashed_password")));
    let sut = DbAddAccount::new(encrypter);
    sut
}

fn make_encrypter<T>(callback: T) -> Box<dyn Encrypter>
where
    T: Fn(&str) -> TError<String> + Send + Sync + 'static,
{
    struct EncrypterStub<T>
    where
        T: Fn(&str) -> TError<String> + Send + Sync,
    {
        callback: T,
    }

    #[async_trait]
    impl<T> Encrypter for EncrypterStub<T>
    where
        T: Fn(&str) -> TError<String> + Send + Sync,
    {
        async fn encrypt(&self, value: &str) -> TError<String> {
            let callback = &self.callback;
            callback(value)
        }
    }

    Box::new(EncrypterStub { callback })
}

#[tokio::test]
async fn calls_encrypter_with_correct_password() {
    let mut sut = make_sut();
    sut.set_encrypter(make_encrypter(|email| {
        assert_eq!(email, "valid_email@mail.com");
        Ok(String::new())
    }));

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    match sut.add(account_dto).await {
        Ok(_) => {}
        Err(_) => {}
    };
}

#[tokio::test]
async fn returns_err_if_encryter_returns_err() {
    let mut sut = make_sut();
    sut.set_encrypter(make_encrypter(|_| Err(Box::new(ErrorMsg::default()))));

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    let result = sut.add(account_dto).await;

    assert!(result.is_err());
}
