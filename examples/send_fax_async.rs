
use std::{fs, error::Error};
use retarus::common::creds::Credentials;
use retarus::fax::document::Document;
use retarus::fax::job::JobBuilder;
use retarus::fax::client::ClientSDK;
use retarus::common::uri::Region;


fn read_file() -> Result<Option<Document>, Box<dyn Error>>{
    // Here we read a file and for simplicity, we also create a document object that will be returned.
    let dir = fs::read_dir("./assets")?;
    for entry in dir{
        let en = entry?;
        
        // read the file
        let result = fs::read(en.path())?;
        let filename = en.file_name().into_string().unwrap();

        // create a new document object with the data from the two variables above
        let doc = Document::new(filename, result, None);

        return Ok(Some(doc))
    }
    Ok(None)
}


#[tokio::main]
async fn main() {
    // First we need to create a credentials object to authenticate ourselves.

    let user_id = std::env::var("retarus_userid").unwrap();
    let password = std::env::var("retarus_fax_password").unwrap();
    
    let customer_number = std::env::var("retarus_customer_number").unwrap();


    // Now let's create a client
    let client = ClientSDK::builder()
    .set_customer_number(customer_number)
    .set_region(Region::Europe)
    .set_credentials(Credentials::new(user_id.as_str(), password.as_str()))
    .build();
    
    // Now let's read a file of a directory.
    let data = read_file().unwrap();


    let mut job_id = String::new();
    // if read was successful, we send the fax to the Retarus server
    if data.is_some() {
        let job = JobBuilder::default()
        .add_document(data.unwrap())
        .add_recipient("+49000000000".to_string())
        .build();
        
        // checks if the fax was successful submitted
        let res = match client.send_job(None, job).await{
            Ok(res) => res,
            Err(err) => panic!("{}", err)
        };
        job_id = res.job_id.clone();
        println!("{:?}", res);
    } else{
        println!("Nothing found to send")
    }

    let res = client.get_fax_report(job_id, None).await;
    println!("{:?}", res);
}