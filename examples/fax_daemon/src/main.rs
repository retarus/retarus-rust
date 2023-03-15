use notify::{watcher, DebouncedEvent, RecursiveMode, Watcher};
use retarus::fax::document::Document;
use retarus::fax::job::Job;
use retarus::fax::report::Report;
use retarus::fax::client::*;
use retarus::common::creds::Credentials;
use retarus::common::transport::blocking;
use retarus::common::uri::Region;
use std;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::sync::mpsc::channel;
use std::time::Duration;
use std::{thread};


fn is_string_numeric(str: &&str) -> bool {
    for c in str.chars() {
        if !c.is_numeric() {
            return false;
        }
    }
    return true;
}

fn create_job(path: PathBuf) -> Result<Option<Job>, Box<dyn std::error::Error>> {
    let x = path.to_str().unwrap().split("/").collect::<Vec<&str>>();
    let filename = x.last().unwrap();
    if filename.contains("_") {
        let y = filename.split("_").collect::<Vec<&str>>();
        if is_string_numeric(y.first().unwrap()) && y.last().unwrap().contains(".pdf") {
            // Read content of pdf file
            let data = std::fs::read(path.clone()).unwrap();

            // create a new document instance
            let doc = Document::new(y.last().unwrap().to_string(), data, None);

            // Create a job
            let job = Job::builder()
                .add_document(doc)
                .add_recipient(y.first().unwrap().to_string())
                .build();
            return Ok(Some(job));
        }
    }
    return Ok(None);
}

fn write_report(report: Report) {
    // write the report to the in directory
    let mut file = File::create(format!("in/{}.json", report.job_id)).unwrap();
    let a = serde_json::to_string(&report).unwrap();
    file.write_all(a.as_bytes()).unwrap();
    println!("Wrote report");
}

fn main() {
    let (tx, rx) = channel();

    // Create a watcher object, delivering debounced events.
    // The notification back-end is selected based on the platform.
    let mut watcher = watcher(tx, Duration::from_secs(10)).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch("out", RecursiveMode::Recursive).unwrap();


    // Load the credentials from environment variables
    let user_id = std::env::var("retarus_userid").unwrap();
    let password = std::env::var("retarus_fax_password").unwrap();
    
    let customer_number = std::env::var("retarus_customer_number").unwrap();

    // create a sdk instance with all needed parameters
    let sdk = ClientSDK::builder()
        .set_region(Region::Europe)
        .set_customer_number(customer_number)
        .set_credentials(Credentials::new(user_id.as_str(), password.as_str()))
        .build();

    loop {
        // Check for file creations in the out directory
        let res = match rx.recv() {
            Ok(event) => event,
            Err(e) => panic!("watch error: {:?}", e),
        };
        match res {
            // Detected event
            DebouncedEvent::Create(path) => {
                let job = create_job(path).unwrap().unwrap();

                // send the fax
                let res = blocking(sdk.send_job(None, job)).unwrap();
                println!("Created and sent Fax");
                let job_id = res.job_id;
                let mut is_processed = false;
                // wait until the job has been processed to create a local copy of the fax report
                while is_processed == false {
                    let report_res = blocking(sdk.get_fax_report(job_id.clone(), None)).unwrap();
                    if report_res
                        .recipient_status
                        .get(0)
                        .unwrap()
                        .clone()
                        .unwrap()
                        .status
                        != "PENDING"
                    {
                        write_report(report_res);
                        is_processed = true;
                    }
                    thread::sleep(Duration::from_secs(40));
                }
            }
            _ => {}
        }
    }
}
