use retarus::{sms::{client::{SmsClient}, models::{SmsJob, SmsFilter}}, general::transport::blocking};

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

#[test]
fn send_sms() {
    let client = create_client();
    let job = SmsJob::builder()
        .add_message("Hallo Welt".to_string(), vec!["490000000000".to_string()])
        .build();

    let res = blocking(client.send_sms(job)).unwrap();
    assert_ne!(res.job_id, String::new());
    
    let report = blocking(client.get_sms_job(res.job_id.clone())).unwrap();

    assert_eq!(report.job_id, res.job_id);
}

#[test]
fn test_filter_sms_jobs() {
    let client= create_client();
    let filter = SmsFilter::builder().only_job_ids(true).build();
    let res = blocking(client.filter_sms_jobs(filter)).unwrap();
    assert_eq!(res[0].src, None)
}