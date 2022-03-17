use async_trait::async_trait;

use crate::domain::entities::account::AccountEntity;

#[async_trait]
pub trait AddAccount: Send + Sync {
    async fn add(&self, account_dto: AddAccountDto) -> AccountEntity;
}

pub struct AddAccountDto {
    pub name: String,
    pub email: String,
    pub password: String,
}
