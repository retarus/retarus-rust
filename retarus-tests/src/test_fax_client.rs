use std::time::Duration;

use retarus::{general::job::Job, fax::{_async::client::ClientSDK, responses::{BulkGet, BulkDelete}}};

use crate::provider::{provide_test_credentials, provide_test_file};


fn create_client() -> ClientSDK {
    let client = ClientSDK::builder()
        .set_customer_number("99999".to_string())
        .set_credentiale(provide_test_credentials())
        .build();
    return client;
}

async fn send_fax() -> String {
    let client = create_client();
    let job = Job::builder()
        .add_document(provide_test_file())
        .add_recipient("49000000000".to_string())
        .build();
    let res = client.send_job(None, job).await.unwrap();
    if res.job_id == "".to_string() {
        assert!(false, "Failed to send fax, did not get a job id")
    }
    println!("{}", res.job_id.clone());
    return res.job_id
}

#[tokio::test]
async fn test_get_fax_report() {
    let ji = send_fax().await;
    let client = create_client();
    let res = client.get_fax_report(ji, None).await;
    println!("{:?}", res);
    let res = res.unwrap();
    if res.job_id == String::new() {
        assert!(false, "Eventually result wrong parsed")
    }
}

#[tokio::test]
async fn test_delete_fax_report() {
    let ji = send_fax().await;
    let client = create_client();
    let res = client.delete_fax_report(ji, None).await.unwrap();
    if res.deleted == false{
        assert!(false, "failed, because report was not deleted")
    }
}


#[tokio::test]
async fn test_bulk_operations() {
    let ji = send_fax().await;
    let client = create_client();
    //let payload = ReportsAction{action: "GET".to_string(), job_ids: vec![ji]};
    let job_ids = vec![ji];
    let res = client.perform_bulk_get(job_ids.clone(), None).await;
    println!("{:?}", res);
    let res = res.unwrap();
    if res.reports.len() == 0 {
        assert!(false, "No fax reports found")
    }
    let res = client.perform_bulk_delete(job_ids, None).await.unwrap();
    println!("{:?}", res);
    if res.reports.len() == 0 {
        assert!(false, "No fax reports deleted")
    }
}