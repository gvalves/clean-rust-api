use mockall::predicate;

use crate::domain::usecases::add_account::AddAccountDto;
use crate::{
    data::protocols::add_account_repository::AddAccountRepository,
    domain::entities::account::AccountEntity,
};

use super::{AccountMongoRepository, MockStdAccountRepository};

macro_rules! repository_add_default {
    () => {
        |account_dto| {
            let AddAccountDto {
                name,
                email,
                password,
            } = &account_dto;

            Ok(AccountEntity::new("valid_id", name, email, password))
        }
    };
}

fn make_sut() -> AccountMongoRepository {
    let mut repository = make_repository();
    repository.expect_add().returning(repository_add_default!());

    let mut sut = AccountMongoRepository::new();
    sut.set_repository(repository);

    sut
}

fn make_repository() -> Box<MockStdAccountRepository> {
    Box::new(MockStdAccountRepository::default())
}

#[tokio::test]
async fn calls_repository_implementation_with_correct_data() {
    let mut repository = make_repository();
    repository
        .expect_add()
        .once()
        .with(predicate::eq(AddAccountDto {
            name: String::from("valid_name"),
            email: String::from("valid_email@mail.com"),
            password: String::from("valid_password"),
        }))
        .returning(repository_add_default!());

    let mut sut = make_sut();
    sut.set_repository(repository);

    let account_dto = AddAccountDto {
        name: String::from("valid_name"),
        email: String::from("valid_email@mail.com"),
        password: String::from("valid_password"),
    };

    match sut.add(account_dto).await {
        Ok(_) => {}
        Err(_) => {}
    }
}
