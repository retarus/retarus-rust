
use retarus::{fax::sync::client::ClientSDK, general::{document::Document, job::Job}};

use crate::provider::{provide_test_credentials, provide_test_file};


fn create_sync_client() -> ClientSDK {
    let sdk = ClientSDK::builder()
    .set_customer_number("99999".to_string())
    .set_credentiale(provide_test_credentials())
    .build();
    return sdk;
}


#[test]
fn test_send_fax() {
    let client = create_sync_client();
    let doc = provide_test_file();
    let job = Job::builder()
        .add_document(doc)
        .add_recipient("4900000000001".to_string())
        .build();

    let res = client.send_job(None, job);
    println!("{:?}", res);
}