use async_trait::async_trait;
use tokio;

use crate::data::protocols::{add_account_repository::AddAccountRepository, encrypter::Encrypter};
use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::{AddAccount, AddAccountDto};
use crate::{ErrorMsg, TError};

use super::DbAddAccount;

fn make_sut() -> DbAddAccount {
    let encrypter = make_encrypter(|_| Ok(String::from("hashed_password")));

    let add_account_repository = make_add_account_repository(|account_dto| {
        let AddAccountDto {
            name,
            email,
            password,
        } = account_dto;

        Ok(AccountEntity::new("valid_id", &name, &email, &password))
    });

    let sut = DbAddAccount::new(encrypter, add_account_repository);
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

fn make_add_account_repository<T>(callback: T) -> Box<dyn AddAccountRepository>
where
    T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync + 'static,
{
    struct AddAccountRepositoryStub<T>
    where
        T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync,
    {
        callback: T,
    }

    #[async_trait]
    impl<T> AddAccountRepository for AddAccountRepositoryStub<T>
    where
        T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync,
    {
        async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity> {
            let callback = &self.callback;
            callback(account_dto)
        }
    }

    Box::new(AddAccountRepositoryStub { callback })
}

#[tokio::test]
async fn calls_encrypter_with_correct_password() {
    static mut CALLED: bool = false;

    let mut sut = make_sut();
    sut.set_encrypter(make_encrypter(|email| {
        unsafe { CALLED = true }
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

    assert!(unsafe { CALLED });
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

    if let Some(err) = sut.add(account_dto).await.err() {
        assert_eq!(err.to_string(), ErrorMsg::default().to_string())
    } else {
        assert!(false);
    }
}

#[tokio::test]
async fn calls_add_account_repository_with_correct_data() {
    static mut CALLED: bool = false;

    let mut sut = make_sut();
    sut.set_add_account_repository(make_add_account_repository(|account_dto| {
        unsafe { CALLED = true }

        let AddAccountDto {
            name,
            email,
            password,
        } = account_dto;

        assert_eq!(name, "valid_name");
        assert_eq!(email, "valid_email@mail.com");
        assert_eq!(password, "hashed_password");

        Ok(AccountEntity::new("valid_id", &name, &email, &password))
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

    assert!(unsafe { CALLED })
}

#[tokio::test]
async fn returns_err_if_add_account_repository_returns_err() {
    let mut sut = make_sut();
    sut.set_add_account_repository(make_add_account_repository(|_| {
        Err(Box::new(ErrorMsg::default()))
    }));

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    if let Some(err) = sut.add(account_dto).await.err() {
        assert_eq!(err.to_string(), ErrorMsg::default().to_string())
    } else {
        assert!(false);
    }
}

#[tokio::test]
async fn returns_an_account_on_success() {
    let sut = make_sut();

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    let account = sut.add(account_dto).await.unwrap();

    assert_eq!(account.id(), "valid_id");
    assert_eq!(account.name(), "valid_name");
    assert_eq!(account.email(), "valid_email@mail.com");
    assert_eq!(account.password(), "hashed_password");
}
