use mockall::predicate;
use mockall_double::double;
use tokio;

use crate::domain::entities::account::AccountEntity;
#[double]
use crate::domain::usecases::add_account::AddAccount;
use crate::domain::usecases::add_account::AddAccountDto;
use crate::presentation::http::HttpRequest;
use crate::presentation::protocols::controller::ControllerProtocol;
#[double]
use crate::presentation::protocols::email_validator::EmailValidator;
use crate::ErrorMsg;

use super::{SignUpController, SignUpReqBodyBuilder, SignUpResBody};

macro_rules! email_validator_is_valid_default {
    () => {
        |_| Ok(true)
    };
}

macro_rules! add_account_add_default {
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

fn make_sut() -> SignUpController {
    let mut email_validator = make_email_validator();
    email_validator
        .expect_is_valid()
        .returning(email_validator_is_valid_default!());

    let mut add_account = make_add_account();
    add_account
        .expect_add()
        .returning(add_account_add_default!());

    let email_validator = Box::new(email_validator);
    let add_account = Box::new(add_account);

    SignUpController::new(email_validator, add_account)
}

fn make_email_validator() -> EmailValidator {
    EmailValidator::default()
}

fn make_add_account() -> AddAccount {
    AddAccount::default()
}

#[tokio::test]
async fn returns_400_if_req_body_is_none() {
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
async fn returns_400_if_no_name_is_provided() {
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
async fn returns_400_if_no_email_is_provided() {
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
async fn returns_400_if_no_password_is_provided() {
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
async fn returns_400_if_no_password_confirmation_is_provided() {
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
async fn returns_400_if_password_confirmation_fails() {
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
async fn returns_400_if_invalid_email_is_provided() {
    let mut email_validator = make_email_validator();
    email_validator.expect_is_valid().returning(|_| Ok(false));

    let email_validator = Box::new(email_validator);
    let mut sut = make_sut();
    sut.set_email_validator(email_validator);

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
async fn calls_email_validator_with_correct_email() {
    let mut email_validator = make_email_validator();
    email_validator
        .expect_is_valid()
        .with(predicate::eq("any_email@mail.com"))
        .returning(email_validator_is_valid_default!());

    let email_validator = Box::new(email_validator);
    let mut sut = make_sut();
    sut.set_email_validator(email_validator);

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
async fn returns_500_if_email_validator_returns_err() {
    let mut email_validator = make_email_validator();
    email_validator
        .expect_is_valid()
        .returning(|_| ErrorMsg::default().into());

    let email_validator = Box::new(email_validator);
    let mut sut = make_sut();
    sut.set_email_validator(email_validator);

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
async fn calls_add_account_with_correct_values() {
    let mut add_account = make_add_account();
    add_account
        .expect_add()
        .with(predicate::eq(AddAccountDto {
            name: String::from("any_name"),
            email: String::from("any_email@mail.com"),
            password: String::from("any_password"),
        }))
        .returning(add_account_add_default!());

    let add_account = Box::new(add_account);
    let mut sut = make_sut();
    sut.set_add_account(add_account);

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
async fn returns_500_if_add_account_returns_err() {
    let mut add_account = make_add_account();
    add_account
        .expect_add()
        .with(predicate::eq(AddAccountDto {
            name: String::from("any_name"),
            email: String::from("any_email@mail.com"),
            password: String::from("any_password"),
        }))
        .returning(|_| ErrorMsg::default().into());

    let add_account = Box::new(add_account);
    let mut sut = make_sut();
    sut.set_add_account(add_account);

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
async fn returns_200_if_valid_data_is_provided() {
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
