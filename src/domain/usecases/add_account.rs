use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::AccountEntity;
use crate::GenericResult;

#[automock]
#[async_trait]
pub trait AddAccount: Send + Sync {
    async fn add(&self, account_dto: AddAccountDto) -> GenericResult<AccountEntity>;
}

#[derive(Debug, PartialEq)]
pub struct AddAccountDto {
    pub name: String,
    pub email: String,
    pub password: String,
}
