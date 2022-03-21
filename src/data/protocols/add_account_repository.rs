use async_trait::async_trait;
use mockall::automock;

use crate::domain::entities::AccountEntity;
use crate::domain::usecases::AddAccountDto;
use crate::GenericResult;

#[automock]
#[async_trait]
pub trait AddAccountRepository: Send + Sync {
    async fn add(&self, account_dto: AddAccountDto) -> GenericResult<AccountEntity>;
}
