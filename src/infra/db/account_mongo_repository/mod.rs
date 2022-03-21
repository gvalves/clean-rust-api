use async_trait::async_trait;
use mockall::mock;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

use crate::data::protocols::add_account_repository::AddAccountRepository;
use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::AddAccountDto;
use crate::infra::db::mongo_helper::MongoHelper;
use crate::{ErrorMsg, TError};

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
        let client = MongoHelper::get_client().await;
        let db = client.database("clean-rust-api");
        let account_collection = db.collection::<AccountEntity>("accounts");

        let AddAccountDto {
            name,
            email,
            password,
        } = &account_dto;

        let account = AccountEntity::new("", name, email, password);

        let InsertOneResult { inserted_id, .. } =
            match account_collection.insert_one(account, None).await {
                Ok(val) => val,
                Err(err) => return ErrorMsg::parse(err).into(),
            };

        let id = inserted_id.to_string();

        let filter = doc! { "_id": inserted_id };

        let find_result = match account_collection.find_one(filter, None).await {
            Ok(val) => val.unwrap(),
            Err(err) => return ErrorMsg::parse(err).into(),
        };

        Ok(AccountEntity::new(
            &id,
            find_result.name(),
            find_result.email(),
            find_result.password(),
        ))
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
