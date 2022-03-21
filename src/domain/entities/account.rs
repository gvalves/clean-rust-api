use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct AccountEntity {
    id: String,
    name: String,
    email: String,
    password: String,
}

impl AccountEntity {
    pub fn new(id: &str, name: &str, email: &str, password: &str) -> Self {
        let id = String::from(id);
        let name = String::from(name);
        let email = String::from(email);
        let password = String::from(password);

        Self {
            id,
            name,
            email,
            password,
        }
    }

    /// Get a reference to the account entity's id.
    pub fn id(&self) -> &str {
        self.id.as_ref()
    }

    /// Get a reference to the account entity's name.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Get a reference to the account entity's email.
    pub fn email(&self) -> &str {
        self.email.as_ref()
    }

    /// Get a reference to the account entity's password.
    pub fn password(&self) -> &str {
        self.password.as_ref()
    }
}
