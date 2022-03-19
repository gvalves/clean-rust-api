use async_trait::async_trait;

use crate::{
    domain::{entities::account::AccountEntity, usecases::add_account::AddAccountDto},
    TError,
};

#[async_trait]
pub trait AddAccountRepository: Send + Sync {
    async fn add(&self, account_dto: AddAccountDto) -> TError<AccountEntity>;
}
