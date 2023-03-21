#[cfg(test)]
use retarus::{
    common::transport::blocking,
    sms::models::{SmsFilter, SmsJob},
};
#[cfg(test)]
use std::time::Duration;

#[cfg(test)]
use test_utils::create_client;


#[cfg(test)]
mod test_utils {
    use retarus::sms::client::SmsClient;

    use crate::provider::provide_sms_credentials;

    pub fn create_client() -> SmsClient {
        let client = SmsClient::builder()
            .set_credentials(provide_sms_credentials())
            .build();
        return client;
    }
}
#[test]
fn test_fetch_sms_report() {
    let client = create_client();
    let res = blocking(client.get_sms_job(String::new()));
    println!("{:?}", res);
    if res.is_ok() {
        assert!(false, "Report was found, wrong behavior")
    }
}
#[cfg(test)]
#[test]
fn send_sms() {
    let client = create_client();
    let job = SmsJob::builder()
        .add_message("Hallo Welt".to_string(), vec!["490000000000".to_string()])
        .build();

    let res = blocking(client.send_sms(job)).unwrap();
    assert_ne!(res.job_id, String::new());

    std::thread::sleep(Duration::from_secs(4));

    let report = blocking(client.get_sms_job(res.job_id.clone())).unwrap();

    assert_eq!(report.job_id, res.job_id);
}