#[cfg(test)]
use retarus::{webexpress::{client::WebexpressClient, job::WebexpressJob}, common::creds::Credentials};#[cfg(test)]



#[tokio::test]
async fn test_create_client() {
    let username = std::env::var("webexpress_username").unwrap();
    let password = std::env::var("webexpress_password").unwrap();
    let client = WebexpressClient::builder().set_credentiale(Credentials::new(username.as_str(), password.as_str())).build();
    let job = WebexpressJob::minimal("RetarusCustomerTest".to_string(), "Customers with a high order volume".to_string(), "distributionlist".to_string(), "assets/test.csv".to_string(), "utf-8".to_string(), "+49".to_string());
    let res = client.upload_distributor_list(job).await.unwrap();
    println!("{:?}", res);
    assert!(res.contains("The list has been successfully imported"))
}