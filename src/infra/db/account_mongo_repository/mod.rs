use async_trait::async_trait;
use mockall::mock;

use crate::data::protocols::add_account_repository::AddAccountRepository;
use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::AddAccountDto;
use crate::TError;

use super::protocols::account_repository::AccountRepository;

#[cfg(test)]
mod tests;

pub struct AccountMongoRepository {
    repository: Box<dyn AccountRepository>,
}

impl AccountMongoRepository {
    pub fn new() -> Self {
        Self {
            repository: Box::new(StdAccountRepository),
        }
    }

    /// Set the account mongo repository's repository.
    pub fn set_repository(&mut self, repository: Box<dyn AccountRepository>) {
        self.repository = repository;
    }
}

#[async_trait]
impl AddAccountRepository for AccountMongoRepository {
    async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity> {
        self.repository.add(account_dto).await
    }
}

struct StdAccountRepository;

#[async_trait]
impl AddAccountRepository for StdAccountRepository {
    async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity> {
        todo!()
    }
}

impl AccountRepository for StdAccountRepository {}

mock! {
    StdAccountRepository {}

    #[async_trait]
    impl AddAccountRepository for StdAccountRepository {
        async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity>;
    }

    impl AccountRepository for StdAccountRepository {}
}
