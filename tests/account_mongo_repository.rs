mod add {
    use clean_rust_api::data::protocols::AddAccountRepository;
    use clean_rust_api::domain::usecases::AddAccountDto;
    use clean_rust_api::infra::db::AccountMongoRepository;

    #[tokio::test]
    async fn returns_an_account_on_success() {
        let sut = AccountMongoRepository::new();
        let account_dto = AddAccountDto {
            name: String::from("Foo"),
            email: String::from("foo@gmail.com"),
            password: String::from("123"),
        };

        let account = sut.add(account_dto).await.unwrap();

        assert!(!account.id().is_empty());
        assert_eq!(account.name(), "Foo");
        assert_eq!(account.email(), "foo@gmail.com");
        assert_eq!(account.password(), "123");
    }
}
