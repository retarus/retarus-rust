use std::{error::Error};

use hyper::StatusCode;

use crate::{
    common::{
        creds::Credentials,
        transport::{response_to_body, Transporter},
        uri::{ Region, RegionUri},
    },
    sms::models::{SmsJob},
};

use super::models::{JobReport, JobResponse};

/// The official Retarus SMS SDK client, use the [SmsClientBuilder] function to configure an instance.
/// 
/// ## Example
/// ```ignore
/// use retarus::common::creds::Credentials;
/// use retarus::sms::client::{SmsClient};
/// 
/// let sdk = SmsClient::builder()
/// .set_credentials(Credentials::from_env()
/// .expect("You need to export your credentials"))
/// .build();
/// ```
pub struct SmsClient {
    transporter: Transporter,
    region_uri: RegionUri,
}
impl SmsClient {
    //! Create a builder instance of SmsClientBuilder, which you can use to configure from example: Set a specific region.
    pub fn builder() -> SmsClientBuilder {
        SmsClientBuilder {
            region: Region::Europe,
            region_uris: vec![RegionUri::new(
                Region::Europe,
                "https://sms4a.eu.retarus.com",
                vec![
                    "https://sms4a.de1.retarus.com",
                    "https://sms4a.de2.retarus.com",
                ],
            )],
            credentails: Credentials::new("", ""),
        }
    }
}
impl SmsClient {
    /// Takes a SmsJob instance and send a sms according to the specified details to the Retarus servers to be processed.
    /// Returns: the job_id
    pub async fn send_sms(&self, job: SmsJob) -> Result<JobResponse, Box<dyn Error>> {
        let uri = format!("{}/rest/v1/jobs", &self.region_uri.ha_addr);
        let res = self.transporter.post::<SmsJob>(uri, job).await?;
        if res.status() != StatusCode::OK && res.status() != StatusCode::CREATED {
            let a = response_to_body(res).await?;
            return Err(a.into())
        }
        let a = response_to_body(res).await?;
        let x: JobResponse = serde_json::from_str(a.as_str())?;
        Ok(x)
    }

    /// Get a specific job from the server
    pub async fn get_sms_job(&self, job_id: String) -> Result<JobReport, Box<dyn Error>> {
        for server in &self.region_uri.servers {
            let uri = format!("{}/rest/v1/jobs/{}", server, job_id);
            let res = self.transporter.get(uri).await?;
            if res.status() != StatusCode::OK {
                continue;
            }
            let a = response_to_body(res).await?;
            let x: JobReport = serde_json::from_str(a.as_str())?;
            return Ok(x);
        }
        Err("No report found!".into())
    }
}




pub struct SmsClientBuilder {
    region: Region,
    region_uris: Vec<RegionUri>,
    credentails: Credentials,
}
impl SmsClientBuilder {
    pub fn set_credentials(mut self, credentails: Credentials) -> SmsClientBuilder {
        self.credentails = credentails;
        self
    }
    pub fn set_region(mut self, region: Region) -> SmsClientBuilder {
        self.region = region;
        self
    }
    /// The builder function validates the given parameter and will return a instance of [SmsClient].
    pub fn build(self) -> SmsClient {
        assert!(
            self.credentails.password != "",
            "Credentials must be set to use the sms client"
        );
        assert!(self.region == Region::Europe, "The sms service is currently only processed in the European region, select Region::Europe as your region.");

        return SmsClient {
            transporter: Transporter::new(self.credentails),
            region_uri: self.region_uris.first().unwrap().to_owned()
        };
    }
}
