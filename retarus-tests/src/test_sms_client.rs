use retarus::sms::client::{SmsClient, blocking};

use crate::provider::provide_sms_credentials;


fn create_client() -> SmsClient {
    let client = SmsClient::builder()
    .set_credentiale(provide_sms_credentials())
    .build();
    return client;
}

#[test]
fn test_fetch_sms_report() {
    let client = create_client();
    let res = blocking(client.get_sms_job(String::new()));
    println!("{:?}", res);
    if res.is_ok(){
        assert!(false, "Report was found, wrong behavior")
    }
}