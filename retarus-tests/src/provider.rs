use retarus::{general::{creds::Credentials}, fax::document::Document};
use std::env;


pub fn provide_test_file() -> Document {
    let data = std::fs::read("assets/testPdf.pdf").unwrap();
    let docs = Document::new("test.pdf".to_string(), data, None);
    return docs;
}



pub fn provide_test_credentials() -> Credentials {
    Credentials::new(env::var("retarus_userid").unwrap().as_str(), env::var("retarus_fax_password").unwrap().as_str())
}
pub fn provide_sms_credentials() -> Credentials {
    Credentials::new(env::var("retarus_userid").unwrap().as_str(), env::var("retarus_sms_password").unwrap().as_str())
}

pub fn provide_customer_number() -> String {
    env::var("retarus_customer_number").unwrap()
}