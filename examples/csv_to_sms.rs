use csv::Reader;
use retarus::general::creds::Credentials;
use retarus::general::transport::blocking;
use retarus::general::uri::Region;
use retarus::sms::client::{SmsClient};
use retarus::sms::models::SmsJob;

use std::fs;

fn read_ad_text() -> String {
    let path = "assets/advertisement.txt";
    let data = fs::read_to_string(path).expect("Could not read advertisement text");
    return data;
}

fn load_list(to_send: String) -> Vec<SmsJob> {
    let mut jobs = vec![];

    let path = "assets/sms_data.csv";
    let mut rdr = Reader::from_path(path).expect("Could not read file");

    for entry in rdr.records() {
        let data = entry.unwrap();
        let filled = to_send.replace("{}", &data[1]);
        let job = SmsJob::builder()
            .add_message(filled, vec![data[2].to_string()])
            .build();

        jobs.push(job);
    }
    return jobs;
}

fn main() {
    let ad = read_ad_text();
    let jobs = load_list(ad);

    let sdk = SmsClient::builder()
        .set_region(Region::Europe)
        .set_credentiale(Credentials::from_env().expect("You need to export your credentials"))
        .build();

    let mut ids = vec![];

    for job in jobs {
        let res = blocking(sdk.send_sms(job)).expect("Error while sending sms job");
        ids.push(res.job_id);
    }

    println!("Send all sms")

}
