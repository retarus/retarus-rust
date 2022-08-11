use std::error::Error;
use hyper::Method;
use crate::fax::responses::{BulkDelete, BulkGet, DeletedReport, FaxJobResponse};
use crate::general::creds::Credentials;
use crate::general::job::Job;
use crate::general::report::{Report, ReportsAction};
use crate::general::transport::Transporter;
use crate::general::uri::{Region, determine_region_uri};

pub struct ClientSDK {
    transporter: Transporter,
    customer_number: String,
}
// todo document the functions, what they do, how to use them (Example)

impl ClientSDK {
    /// Create a ClientSDK instance with builder, just set the nessecary parameters and you are ready to go:
    /// ## Example
    /// ```rust
    /// use retarus::fax::_async::client::ClientSDK;
    /// use retarus::general::creds::Credentials;
    ///
    /// let creds = Credentials::new("your_user_id", "your password");
    /// let client = ClientSDK::builder()
    /// .set_customer_number("customer_number".to_string())
    /// .set_credentiale(creds)
    /// .build();
    /// ```
    pub fn builder() -> ClientSDKBuilder {
        ClientSDKBuilder::default()
    }

    /// Send a fax to the retarus server.
    pub async fn send_job(
        &self,
        customer_number: Option<String>,
        job: Job,
    ) -> Result<FaxJobResponse, Box<dyn std::error::Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }

        return self.transporter.clone().send_job(job, cn).await;
    }


    /// Get the fax report for a specific job via the job_id that was returned from the send_job function.
    ///
    /// customer_number is optional and can be used if a job is from another customer_number, overrides the default.
    pub async fn get_fax_report(
        &self,
        job_id: String,
        customer_number: Option<String>,
    ) -> Result<Report, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        return self
            .transporter.clone()
            .report_action::<Report>(Method::GET, cn, job_id)
            .await;
    }

    /// Delete a single fax report with job_id.
    pub async fn delete_fax_report(
        &self,
        job_id: String,
        customer_number: Option<String>,
    ) -> Result<DeletedReport, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let a = self
            .transporter.clone()
            .report_action::<DeletedReport>(Method::DELETE, cn, job_id)
            .await;
        return a;
    }

    /// Fetches the last fax_reports limit = 1000.
    pub async fn get_fax_reports(
        &self,
        customer_number: Option<String>,
    ) -> Result<BulkGet, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let a = self
            .transporter.clone()
            .bulk_action::<String, BulkGet>(Method::GET, cn, None)
            .await;
        return a;
    }

    /// Takes a vector of job_ids and deletes the corresponding report on the retaurs servers.
    pub async fn perform_bulk_delete(&self, job_ids: Vec<String>, customer_number: Option<String>) -> Result<BulkDelete, Box<dyn Error>>{
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let payload = ReportsAction { action: "DELETE".to_string(), job_ids };
        let a = self
        .transporter.clone()
        .bulk_action::<ReportsAction, BulkDelete>(Method::POST, cn, Some(payload))
        .await;
        return a;
    }

    /// Takes a vector of job_ids and returns the report for that numbers, if no report is found it will throw an error.
    pub async fn perform_bulk_get(&self, job_ids: Vec<String>, customer_number: Option<String>) -> Result<BulkGet, Box<dyn Error>>{
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let payload = ReportsAction { action: "GET".to_string(), job_ids };
        let a = self
        .transporter.clone()
        .bulk_action::<ReportsAction, BulkGet>(Method::POST, cn, Some(payload))
        .await;
        return a;
    }

    /// Delete all reports(up to 1000 with one call). If you want to delete specific reports with the job_id, then use the [perform_bulk_delete] function
    pub async fn prune_reports(
        &self,
        customer_number: Option<String>,
    ) -> Result<BulkDelete, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let a = self
            .transporter.clone()
            .bulk_action::<String, BulkDelete>(Method::DELETE, cn, None)
            .await;
        return a;
    }
}

#[derive(Debug, Clone)]
pub struct ClientSDKBuilder {
    region: Region,
    credentials: Credentials,
    customer_number: String,
}
impl<'n> ClientSDKBuilder {
    fn default() -> ClientSDKBuilder {
        ClientSDKBuilder {
            region: Region::Europe,
            credentials: Credentials::default(),
            customer_number: "".to_string(),
        }
    }

    pub fn set_credentiale(mut self, credentials: Credentials) -> ClientSDKBuilder {
        self.credentials = credentials;
        self
    }

    pub fn set_region(mut self, region: Region) -> ClientSDKBuilder {
        self.region = region;
        self
    }

    pub fn set_customer_number(mut self, customer_number: String) -> ClientSDKBuilder {
        self.customer_number = customer_number;
        self
    }

    /// Build a new [ClientSDK] instance with the the arguments given to the builder. 
    pub fn build(self) -> ClientSDK {
        return ClientSDK {
            transporter: Transporter::new(determine_region_uri(self.region), self.credentials),
            customer_number: self.customer_number.to_owned(),
        };
    }
}

#[test]
fn test_build_client() {
    let creds = Credentials::new("abc", "password123");
    let client = ClientSDKBuilder::default().set_credentiale(creds);
    let a = client.build();
}
