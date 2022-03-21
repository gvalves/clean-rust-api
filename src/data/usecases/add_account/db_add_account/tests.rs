use mockall::predicate;
use mockall_double::double;

#[double]
use crate::data::protocols::add_account_repository::AddAccountRepository;
#[double]
use crate::data::protocols::encrypter::Encrypter;
use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::{AddAccount, AddAccountDto};
use crate::ErrorMsg;

use super::DbAddAccount;

macro_rules! encrypter_encrypt_default {
    () => {
        |_| Ok(String::from("hashed_password"))
    };
}

macro_rules! add_account_repository_add_default {
    () => {
        |account_dto| {
            let AddAccountDto {
                name,
                email,
                password,
            } = &account_dto;

            Ok(AccountEntity::new("valid_id", name, email, password))
        }
    };
}

fn make_sut() -> DbAddAccount {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .returning(encrypter_encrypt_default!());

    let mut add_account_repository = make_add_account_repository();
    add_account_repository
        .expect_add()
        .returning(add_account_repository_add_default!());

    let sut = DbAddAccount::new(encrypter, add_account_repository);
    sut
}

fn make_encrypter() -> Box<Encrypter> {
    Box::new(Encrypter::default())
}

fn make_add_account_repository() -> Box<AddAccountRepository> {
    Box::new(AddAccountRepository::default())
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

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    match sut.add(account_dto).await {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[tokio::test]
async fn calls_encrypter_with_correct_password() {
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .with(predicate::eq("valid_password"))
        .returning(encrypter_encrypt_default!());

    let mut sut = make_sut();
    sut.set_encrypter(encrypter);

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
    let mut encrypter = make_encrypter();
    encrypter
        .expect_encrypt()
        .with(predicate::eq("valid_password"))
        .returning(|_| ErrorMsg::default().into());

    let mut sut = make_sut();
    sut.set_encrypter(encrypter);

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
async fn calls_add_account_repository() {
    let mut add_account_repository = make_add_account_repository();
    add_account_repository
        .expect_add()
        .once()
        .returning(add_account_repository_add_default!());

    let mut sut = make_sut();
    sut.set_add_account_repository(add_account_repository);

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    match sut.add(account_dto).await {
        Ok(_) => {}
        Err(_) => {}
    }
}

#[tokio::test]
async fn calls_add_account_repository_with_correct_data() {
    let mut add_account_repository = make_add_account_repository();
    add_account_repository
        .expect_add()
        .with(predicate::eq(AddAccountDto {
            name: String::from("valid_name"),
            email: String::from("valid_email@mail.com"),
            password: String::from("hashed_password"),
        }))
        .returning(add_account_repository_add_default!());

    let mut sut = make_sut();
    sut.set_add_account_repository(add_account_repository);

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
async fn returns_err_if_add_account_repository_returns_err() {
    let mut add_account_repository = make_add_account_repository();
    add_account_repository
        .expect_add()
        .once()
        .returning(|_| ErrorMsg::default().into());

    let mut sut = make_sut();
    sut.set_add_account_repository(add_account_repository);

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
