use std::{env, error::Error};


/// the Credentails struct should contain the username and password to authorize the requests sent to the server.
/// 
/// ## Example
/// ```rust
/// use retarus::common::creds::Credentials;
/// 
/// let creds = Credentials::new("abc", "password123");
/// ´´´
#[derive(Debug, Clone, PartialEq)]
pub struct Credentials {
    pub username: String,
    pub password: String
}
impl <'a> Credentials {
    pub fn new(username: &'a str, password: &'a str) -> Credentials {
        Credentials { username: username.to_string(), password: password.to_string()}
    }
    pub fn default() -> Credentials {
        Credentials{ username: "exmapleUsername".to_string(), password: "yourPassword".to_string()}
    }

    /// Create a [Credentials] instance from env. Before you use this function, you need to export following keys:
    /// - retarus_userid
    /// - retarus_password
    pub fn from_env() -> Result<Credentials, Box<dyn Error>> {
        let user_id = env::var("retarus_userid")?;
        let password = env::var("retarus_password")?;
        Ok(Credentials{username: user_id, password})
    }

    pub fn encode(self) -> String {
        let to_encode = format!("{}:{}", self.username, self.password);
        base64::encode(to_encode)
    }

}