use std::{fs, error::Error};
use retarus::common::creds::Credentials;
use retarus::fax::document::Document;
use retarus::fax::job::JobBuilder;
use retarus::fax::client::ClientSDK;
use retarus::common::uri::Region;


fn read_file(path: &str) -> Result<Option<Document>, Box<dyn Error>>{
    // Here we read a file and for simplicity, we also create a document object that will be returned.
    let dir = fs::read_dir(path)?;
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
    let user_id = "your username";
    let password = "your password";
    let customer_number = "your customer_id";
    

    // Now let's create a client
    let client = ClientSDK::builder()
        .set_customer_number(customer_number.to_string())
        .set_region(Region::Singapore)
        .set_credentials(Credentials::new(user_id, password))
        .build();
    
    // Now let's read a file of a directory.
    let data = read_file("your file").unwrap();


    let mut job_id = String::new();
    // if read was sucessfull, we send the fax to the Retarus server
    if data.is_some() {
        let job = JobBuilder::default()
        .add_document(data.unwrap())
        .add_recipient("your receipient".to_string())
        .build();
        
        //  checks if the fax was sucessfully submitted
        let res = match client.send_job(None, job).await{
            Ok(res) => res,
            Err(err) => panic!("{}", err)
        };
        job_id = res.job_id.clone();
        println!("{:?}", res);

    } else {
        println!("Nothing found to send")
    }

    // get fax report
    let res = client.get_fax_report(job_id.clone(), None).await;
    println!("{:#?}", res);

    // get all fax reports which are currently available
    let res = client.get_fax_reports(None).await;
    println!("{:#?}", res);

    // delete a fax job
    let res = client.delete_fax_report(job_id, None).await;
    println!("{:#?}", res);

    // Tries to fetch available fax reports, after deleting all of them before.
    let res = client.get_fax_reports(None).await;
    println!("{:#?}", res);
}
