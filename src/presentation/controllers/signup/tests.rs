use async_trait::async_trait;
use tokio;

use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::{AddAccount, AddAccountDto};
use crate::presentation::protocols::controller::ControllerProtocol;
use crate::presentation::{http::HttpRequest, protocols::email_validator::EmailValidator};
use crate::{ErrorMsg, TError};

use super::{SignUpController, SignUpReqBodyBuilder, SignUpResBody};

fn make_sut() -> SignUpController {
    let email_validator = make_email_validator();
    let add_account = make_add_account();
    SignUpController::new(email_validator, add_account)
}

fn make_email_validator() -> Box<dyn EmailValidator> {
    make_email_validator_strategy(|_| Ok(true))
}

fn make_email_validator_strategy<T>(strategy: T) -> Box<dyn EmailValidator>
where
    T: Fn(&str) -> TError<bool> + Send + Sync + 'static,
{
    struct EmailValidatorStub<T>
    where
        T: Fn(&str) -> TError<bool> + Send + Sync,
    {
        strategy: T,
    }

    impl<T> EmailValidator for EmailValidatorStub<T>
    where
        T: Fn(&str) -> TError<bool> + Send + Sync,
    {
        fn is_valid(&self, email: &str) -> TError<bool> {
            let strategy = &self.strategy;
            strategy(email)
        }
    }

    Box::new(EmailValidatorStub { strategy })
}

fn make_add_account() -> Box<dyn AddAccount> {
    make_add_account_strategy(|account_dto| {
        let AddAccountDto {
            name,
            email,
            password,
        } = account_dto;

        Ok(AccountEntity::new("valid_id", &name, &email, &password))
    })
}

fn make_add_account_strategy<T>(strategy: T) -> Box<dyn AddAccount>
where
    T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync + 'static,
{
    struct AddAccountStub<T>
    where
        T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync,
    {
        strategy: T,
    }

    #[async_trait]
    impl<T> AddAccount for AddAccountStub<T>
    where
        T: Fn(AddAccountDto) -> TError<AccountEntity> + Send + Sync,
    {
        async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity> {
            let strategy = &self.strategy;
            strategy(account_dto)
        }
    }

    Box::new(AddAccountStub { strategy })
}

#[tokio::test]
pub async fn returns_400_if_req_body_is_none() {
    let sut = make_sut();
    let req = HttpRequest::new(None);
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing body"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_name_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'name'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_email_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'email'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_no_password_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password_confirmation("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'password'"))
    );
}
#[tokio::test]
pub async fn returns_400_if_no_password_confirmation_is_provided() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("missing param 'password_confirmation'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_password_confirmation_fails() {
    let sut = make_sut();
    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("invalid_password")
        .build();
    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("invalid param 'password_confirmation'"))
    );
}

#[tokio::test]
pub async fn returns_400_if_invalid_email_is_provided() {
    let mut sut = make_sut();
    sut.set_email_validator(make_email_validator_strategy(|_| Ok(false)));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("invalid_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 400);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("invalid param 'email'"))
    );
}

#[tokio::test]
pub async fn calls_email_validator_with_correct_email() {
    let mut sut = make_sut();
    sut.set_email_validator(make_email_validator_strategy(|email| {
        assert_eq!(email, "any_email@mail.com");
        Ok(false)
    }));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    sut.handle(req).await;
}

#[tokio::test]
pub async fn returns_500_if_email_validator_returns_err() {
    let mut sut = make_sut();
    sut.set_email_validator(make_email_validator_strategy(|_| {
        Err(Box::new(ErrorMsg::default()))
    }));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 500);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("internal server error"))
    );
}

#[tokio::test]
pub async fn calls_add_account_with_correct_values() {
    let mut sut = make_sut();
    sut.set_add_account(make_add_account_strategy(|account_dto| {
        let AddAccountDto {
            name,
            email,
            password,
        } = account_dto;

        assert_eq!(name, "any_name");
        assert_eq!(email, "any_email@mail.com");
        assert_eq!(password, "any_password");

        Ok(AccountEntity::new("any_id", &name, &email, &password))
    }));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    sut.handle(req).await;
}

#[tokio::test]
pub async fn returns_500_if_add_account_returns_err() {
    let mut sut = make_sut();
    sut.set_add_account(make_add_account_strategy(|_| {
        Err(Box::new(ErrorMsg::default()))
    }));

    let body = SignUpReqBodyBuilder::new()
        .set_name("any_name")
        .set_email("any_email@mail.com")
        .set_password("any_password")
        .set_password_confirmation("any_password")
        .build();

    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 500);
    assert_eq!(
        res.body(),
        &SignUpResBody::Err(ErrorMsg::new("internal server error"))
    );
}

#[tokio::test]
pub async fn returns_200_if_valid_data_is_provided() {
    let sut = make_sut();

    let body = SignUpReqBodyBuilder::new()
        .set_name("valid_name")
        .set_email("valid_email@mail.com")
        .set_password("valid_password")
        .set_password_confirmation("valid_password")
        .build();

    let req = HttpRequest::new(Some(body));
    let res = sut.handle(req).await;

    assert_eq!(res.status_code(), 200);

    if let SignUpResBody::Account(account) = res.body() {
        assert_eq!(account.id(), "valid_id");
        assert_eq!(account.name(), "valid_name");
        assert_eq!(account.email(), "valid_email@mail.com");
        assert_eq!(account.password(), "valid_password");
    } else {
        assert!(false)
    }
}
