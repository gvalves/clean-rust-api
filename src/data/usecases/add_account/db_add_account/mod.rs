use std::error;

use async_trait::async_trait;

use crate::{
    data::protocols::encrypter::Encrypter,
    domain::{
        entities::account::AccountEntity,
        usecases::add_account::{AddAccount, AddAccountDto},
    },
    ErrorMsg,
};

#[cfg(test)]
pub mod tests;

pub struct DbAddAccount {
    encrypter: Box<dyn Encrypter>,
}

impl DbAddAccount {
    pub fn new(encrypter: Box<dyn Encrypter>) -> Self {
        Self { encrypter }
    }

    /// Set the db add account's encrypter.
    pub fn set_encrypter(&mut self, encrypter: Box<dyn Encrypter>) {
        self.encrypter = encrypter;
    }
}

#[async_trait]
impl AddAccount for DbAddAccount {
    async fn add(
        &self,
        account_dto: AddAccountDto,
    ) -> Result<AccountEntity, Box<dyn error::Error>> {
        self.encrypter.encrypt(&account_dto.email);

        Err(Box::new(ErrorMsg::new("unimplemented")))
    }
}
