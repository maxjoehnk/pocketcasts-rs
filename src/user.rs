#[derive(Debug, Deserialize, Clone, Default)]
pub struct User {
    pub email: String,
    pub password: String
}

impl User {
    pub fn new<S: Into<String>>(email: S, password: S) -> User {
        User {
            email: email.into(),
            password: password.into()
        }
    }
}