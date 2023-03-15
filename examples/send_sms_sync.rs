use retarus::{sms::{client::{SmsClient}, models::SmsJob}, common::{creds::Credentials, uri::Region, transport::blocking}};


fn main() {
    // load exported credentials
    let user_id = std::env::var("retarus_sms_userid").unwrap();
    let password = std::env::var("retarus_sms_password").unwrap();

    let credentials = Credentials::new(user_id.as_str(), password.as_str());
    
    // use the builder to create a SmsClient.
    let client = SmsClient::builder().set_credentials(credentials).set_region(Region::Europe).build();

    let destination_numbers = vec!["+4912310000000".to_string()];

    let sms = SmsJob::builder().add_message("Hello World, this is an example how to use the retarus sms service via the rust sdk.".to_string(), destination_numbers).build();
    
    // Use the shipped blocking function to block the thread and wait for the response
    let res = blocking(client.send_sms(sms)).unwrap();
    println!("Server-Response: {:?}", res)
}