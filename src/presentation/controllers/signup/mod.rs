use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::domain::entities::AccountEntity;
use crate::domain::usecases::{AddAccount, AddAccountDto};
use crate::presentation::http::{HttpRequest, HttpResponse};
use crate::presentation::protocols::{ControllerProtocol, EmailValidator};
use crate::ErrorMsg;

#[cfg(test)]
pub mod tests;

pub struct SignUpController {
    email_validator: Box<dyn EmailValidator>,
    add_account: Box<dyn AddAccount>,
}

impl SignUpController {
    pub fn new(email_validator: Box<dyn EmailValidator>, add_account: Box<dyn AddAccount>) -> Self {
        Self {
            email_validator,
            add_account,
        }
    }

    /// Set the sign up controller's email validator.
    pub fn set_email_validator(&mut self, email_validator: Box<dyn EmailValidator>) {
        self.email_validator = email_validator;
    }

    /// Set the sign up controller's add account.
    pub fn set_add_account(&mut self, add_account: Box<dyn AddAccount>) {
        self.add_account = add_account;
    }
}

#[async_trait]
impl ControllerProtocol<SignUpReqBody, SignUpResBody> for SignUpController {
    async fn handle(&self, req: HttpRequest<SignUpReqBody>) -> HttpResponse<SignUpResBody> {
        let body = req.body();

        if body.is_none() {
            return bad_request("missing body");
        }

        let body = body.unwrap();
        let name = body.name();
        let email = body.email();
        let password = body.password();
        let password_confirmation = body.password_confirmation();

        if name.is_empty() {
            return bad_request("missing param 'name'");
        }

        if email.is_empty() {
            return bad_request("missing param 'email'");
        }

        if password.is_empty() {
            return bad_request("missing param 'password'");
        }

        if password_confirmation.is_empty() {
            return bad_request("missing param 'password_confirmation'");
        }

        if password != password_confirmation {
            return bad_request("invalid param 'password_confirmation'");
        }

        match self.email_validator.is_valid(email) {
            Ok(is_valid) => {
                if !is_valid {
                    return bad_request("invalid param 'email'");
                }
            }
            Err(_) => return server_error(),
        }

        let result = self
            .add_account
            .add(AddAccountDto {
                name: name.to_string(),
                email: email.to_string(),
                password: password.to_string(),
            })
            .await;

        let account = match result {
            Ok(account) => account,
            Err(_) => return server_error(),
        };

        HttpResponse::new(200, SignUpResBody::Account(account))
    }
}

fn http_error(status_code: u32, msg: &str) -> HttpResponse<SignUpResBody> {
    HttpResponse::new(status_code, SignUpResBody::Err(ErrorMsg::new(msg)))
}

fn bad_request(msg: &str) -> HttpResponse<SignUpResBody> {
    http_error(400, msg)
}

fn server_error() -> HttpResponse<SignUpResBody> {
    http_error(500, "internal server error")
}

#[derive(Debug, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct SignUpReqBody {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

impl SignUpReqBody {
    /// Get a reference to the sign up req body's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the sign up req body's email.
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    /// Get a reference to the sign up req body's password.
    pub fn password(&self) -> &str {
        self.password.as_ref()
    }

    /// Get a reference to the sign up req body's password confirmation.
    pub fn password_confirmation(&self) -> &str {
        self.password_confirmation.as_ref()
    }
}

pub struct SignUpReqBodyBuilder {
    name: String,
    email: String,
    password: String,
    password_confirmation: String,
}

impl SignUpReqBodyBuilder {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            email: String::new(),
            password: String::new(),
            password_confirmation: String::new(),
        }
    }

    pub fn build(self) -> SignUpReqBody {
        let Self {
            name,
            email,
            password,
            password_confirmation,
        } = self;

        SignUpReqBody {
            name,
            email,
            password,
            password_confirmation,
        }
    }

    /// Set the sign up req body builder's name.
    pub fn set_name(self, name: &str) -> Self {
        let mut this = self;
        this.name = String::from(name);
        this
    }

    /// Set the sign up req body builder's email.
    pub fn set_email(self, email: &str) -> Self {
        let mut this = self;
        this.email = String::from(email);
        this
    }

    /// Set the sign up req body builder's password.
    pub fn set_password(self, password: &str) -> Self {
        let mut this = self;
        this.password = String::from(password);
        this
    }

    /// Set the sign up req body builder's password confirmation.
    pub fn set_password_confirmation(self, password_confirmation: &str) -> Self {
        let mut this = self;
        this.password_confirmation = String::from(password_confirmation);
        this
    }
}

#[derive(Debug, PartialEq)]
pub enum SignUpResBody {
    Account(AccountEntity),
    Err(ErrorMsg),
}
