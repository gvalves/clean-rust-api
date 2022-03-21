use mongodb::{options::ClientOptions, Client};
use once_cell::sync::OnceCell;
use tokio::sync::Mutex;

pub struct MongoHelper;

static CLIENT: OnceCell<Client> = OnceCell::new();
static CLIENT_INITIALIZED: OnceCell<Mutex<bool>> = OnceCell::new();

impl MongoHelper {
    pub async fn get_client() -> &'static Client {
        if let Some(v) = CLIENT.get() {
            return v;
        }

        let initialized_mutex = CLIENT_INITIALIZED.get_or_init(|| Mutex::new(false));
        let mut initialized = initialized_mutex.lock().await;

        if !*initialized {
            let options = ClientOptions::parse("mongodb://root:root@localhost:27017")
                .await
                .expect("Failed to parse mongo client options");

            let client = Client::with_options(options).expect("Failed to create mongo client");

            CLIENT.set(client).expect("Cannot create two mongo clients");

            *initialized = true;
            drop(initialized);
        }

        CLIENT.get().unwrap()
    }
}
