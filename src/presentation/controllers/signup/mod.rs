use async_trait::async_trait;

use crate::{
    presentation::{
        http::{HttpRequest, HttpResponse},
        protocols::{controller::ControllerProtocol, email_validator::EmailValidator},
    },
    ErrorMsg,
};

#[cfg(test)]
pub mod tests;

pub struct SignUpController {
    email_validator: Box<dyn EmailValidator>,
}

impl SignUpController {
    pub fn new(email_validator: Box<dyn EmailValidator>) -> Self {
        Self { email_validator }
    }

    /// Set the sign up controller's email validator.
    pub fn set_email_validator(&mut self, email_validator: Box<dyn EmailValidator>) {
        self.email_validator = email_validator;
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

        if body.name().is_empty() {
            return bad_request("missing param 'name'");
        }

        if body.email().is_empty() {
            return bad_request("missing param 'email'");
        }

        if body.password().is_empty() {
            return bad_request("missing param 'password'");
        }

        if body.password_confirmation().is_empty() {
            return bad_request("missing param 'password_confirmation'");
        }

        if body.password() != body.password_confirmation() {
            return bad_request("invalid param 'password_confirmation'");
        }

        if !self.email_validator.is_valid(body.email()) {
            return bad_request("invalid param 'email'");
        }

        HttpResponse::new(200, SignUpResBody::Account(0))
    }
}

fn bad_request(msg: &str) -> HttpResponse<SignUpResBody> {
    HttpResponse::new(400, SignUpResBody::Err(ErrorMsg::new(msg)))
}

#[derive(Debug, PartialEq, PartialOrd)]
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

#[derive(Debug, PartialEq, PartialOrd)]
pub enum SignUpResBody {
    Account(u32),
    Err(ErrorMsg),
}
