use crate::domain::entities::account::AccountEntity;
use crate::domain::usecases::add_account::AddAccountDto;

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

mod add {
    use mockall::predicate;

    use crate::data::protocols::add_account_repository::AddAccountRepository;
    use crate::domain::entities::account::AccountEntity;
    use crate::domain::usecases::add_account::AddAccountDto;
    use crate::ErrorMsg;

    use super::{make_repository, make_sut};

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

    #[tokio::test]
    async fn returns_err_if_repository_implementation_returns_err() {
        let mut repository = make_repository();
        repository
            .expect_add()
            .returning(|_| ErrorMsg::default().into());

        let mut sut = make_sut();
        sut.set_repository(repository);

        let account_dto = AddAccountDto {
            name: String::from("valid_name"),
            email: String::from("valid_email@mail.com"),
            password: String::from("valid_password"),
        };

        let result = sut.add(account_dto).await;

        assert_eq!(
            result.unwrap_err().to_string(),
            ErrorMsg::default().to_string()
        );
    }

    #[tokio::test]
    async fn returns_an_account_on_success() {
        let sut = make_sut();

        let account_dto = AddAccountDto {
            name: String::from("valid_name"),
            email: String::from("valid_email@mail.com"),
            password: String::from("valid_password"),
        };

        let result = sut.add(account_dto).await;

        if let Ok(account) = result {
            assert_eq!(account.id(), "valid_id");
            assert_eq!(account.name(), "valid_name");
            assert_eq!(account.email(), "valid_email@mail.com");
            assert_eq!(account.password(), "valid_password");
        } else {
            assert!(false);
        }
    }
}
