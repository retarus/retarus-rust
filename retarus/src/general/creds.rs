
/// the Credentails struct should contain the username and password to authorize the requests send to the server.
#[derive(Debug, Clone, PartialEq)]
pub struct Credentials {
    pub username: String,
    pub password: String
}
impl <'a> Credentials {
    pub fn new(username: &'a str, password: &'a str) -> Credentials {
        return Credentials { username: username.to_string(), password: password.to_string()}
    }
    pub fn default() -> Credentials {
        return Credentials{ username: "exmapleUsername".to_string(), password: "yourPassword".to_string()}
    }

    pub fn encode(self) -> String {
        let to_encode = format!("{}:{}", self.username, self.password);
        return base64::encode(to_encode);
    }

}