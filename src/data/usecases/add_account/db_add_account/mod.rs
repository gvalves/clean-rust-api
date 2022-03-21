use async_trait::async_trait;

use crate::data::protocols::{add_account_repository::AddAccountRepository, encrypter::Encrypter};
use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::{AddAccount, AddAccountDto};
use crate::GenericResult;

#[cfg(test)]
pub mod tests;

pub struct DbAddAccount {
    encrypter: Box<dyn Encrypter>,
    add_account_repository: Box<dyn AddAccountRepository>,
}

impl DbAddAccount {
    pub fn new(
        encrypter: Box<dyn Encrypter>,
        add_account_repository: Box<dyn AddAccountRepository>,
    ) -> Self {
        Self {
            encrypter,
            add_account_repository,
        }
    }

    /// Set the db add account's encrypter.
    pub fn set_encrypter(&mut self, encrypter: Box<dyn Encrypter>) {
        self.encrypter = encrypter;
    }

    /// Set the db add account's add account repository.
    pub fn set_add_account_repository(
        &mut self,
        add_account_repository: Box<dyn AddAccountRepository>,
    ) {
        self.add_account_repository = add_account_repository;
    }
}

#[async_trait]
impl AddAccount for DbAddAccount {
    async fn add(&self, account_dto: AddAccountDto) -> GenericResult<AccountEntity> {
        let hashed_password = self.encrypter.encrypt(&account_dto.password).await?;

        self.add_account_repository
            .add(AddAccountDto {
                password: hashed_password,
                ..account_dto
            })
            .await
    }
}
