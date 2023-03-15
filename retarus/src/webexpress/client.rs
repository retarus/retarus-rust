use std::{error::Error};



use crate::common::{creds::Credentials, uri::{RegionUri, Region}, transport::Transporter};

use super::job::WebexpressJob;



pub struct WebexpressClient {
    transporter: Transporter,
    region_uri: RegionUri,
    credentials: Credentials,
}
impl WebexpressClient {
    pub fn builder() -> WebExpressBuilder {
        WebExpressBuilder {
            region: Region::Europe,
            region_uris: vec![RegionUri::new(
                Region::Europe,
                "https://webexpress.retarus.com",
                vec![""]
            )],
            credentails: Credentials::new("", ""),
        }
    }
    pub async fn upload_distributor_list(&self, mut job: WebexpressJob) -> Result<String, Box<dyn Error>> {
        let uri = format!("{}/PicoPortal/autoLogin/listImport", &self.region_uri.ha_addr);
        
        if &self.credentials.username != "" {
            job.j_username = Some(self.credentials.username.to_string());
            job.j_password = Some(self.credentials.password.to_string());
        }
        let res = self.transporter.form_post(uri, job).await;
        let x = res?.text().await?;
        Ok(x)
    }
}

pub struct WebExpressBuilder {
    region: Region,
    region_uris: Vec<RegionUri>,
    credentails: Credentials,
}
impl WebExpressBuilder {
    pub fn set_credentiale(mut self, credentails: Credentials) -> WebExpressBuilder {
        self.credentails = credentails;
        self
    }
    pub fn set_region(mut self, region: Region) -> WebExpressBuilder {
        self.region = region;
        self
    }
    /// The builder function validates the given parameter and will return a instance of [SmsClient].
    pub fn build(self) -> WebexpressClient {
        assert!(
            self.credentails.password != "",
            "Credentials must be set to use the sms client"
        );
        assert!(self.region == Region::Europe, "The sms service is currently only processed in the European region, select Region::Europe as your region.");

        return WebexpressClient {
            transporter: Transporter::new(self.credentails.clone()),
            region_uri: self.region_uris.first().unwrap().to_owned(),
            credentials: self.credentails
        };
    }
}
