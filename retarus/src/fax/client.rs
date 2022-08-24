use crate::fax::job::Job;
use crate::fax::report::{Report, ReportsAction};
use crate::fax::responses::{BulkDelete, BulkGet, DeletedReport, FaxJobResponse};
use crate::general::creds::Credentials;
use crate::general::transport::{response_to_body, Transporter};
use crate::general::uri::{determine_region_uri, Region, RegionUri};
use hyper::{StatusCode};
use std::error::Error;

pub struct ClientSDK {
    transporter: Transporter,
    region_uri: RegionUri,
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
        let uri = format!("{}/rest/v1/{}/fax", &self.region_uri.ha_addr, cn);
        let response = self.transporter.post::<Job>(uri, job).await?;
        if response.status() == StatusCode::OK || response.status() == StatusCode::CREATED {
            let res_str = response_to_body(response).await?;
            let a: FaxJobResponse = serde_json::from_str(res_str.as_str())?;
            Ok(a)
        } else {
            let res_str = response_to_body(response).await?;
            Err(res_str.into())
        }
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
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports/{}",
                server, cn, job_id
            );
            let response = self
                .transporter
                .get(uri)
                .await?;
        
            if response.status() == StatusCode::OK {
                let res_str = response_to_body(response).await?;
                let a: Report = serde_json::from_str(res_str.as_str())?;
                return Ok(a);
            }
            if response.status() == StatusCode::NOT_FOUND {
                continue;
            }
    }
    return Err("No Fax report was found, please try again".into())
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
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports/{}",
                server, cn, job_id
            );
            let response = self
                .transporter
                .delete(uri)
                .await?;
        
            if response.status() == StatusCode::OK {
                let res_str = response_to_body(response).await?;
                let a: DeletedReport = serde_json::from_str(res_str.as_str())?;
                return Ok(a);
            }
            if response.status() == StatusCode::NOT_FOUND {
                continue;
            }
    }
    return Err("No Fax report was found, please try again".into())
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
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports",
                server,
                cn
            );
            let response = self
            .transporter
            .get(uri)
            .await?;

            if response.status() == StatusCode::OK {
                let res_str = response_to_body(response).await?;
                let a: BulkGet = serde_json::from_str(res_str.as_str())?;
                return Ok(a);
            }
        }
        Err("No report found!".into())
        
    }

    /// Takes a vector of job_ids and deletes the corresponding report on the retaurs servers.
    pub async fn perform_bulk_delete(
        &self,
        job_ids: Vec<String>,
        customer_number: Option<String>,
    ) -> Result<BulkDelete, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let payload = ReportsAction {
            action: "DELETE".to_string(),
            job_ids,
        };
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports",
                server,
                cn
            );
        let response = self
            .transporter
            .post::<ReportsAction>(uri, payload.clone())
            .await?;
            if response.status() == StatusCode::OK {
                let res_str = response_to_body(response).await?;
                let a: BulkDelete = serde_json::from_str(res_str.as_str())?;
                return Ok(a);
                }
    }
    Err("No report found!".into())
}

    /// Takes a vector of job_ids and returns the report for that numbers, if no report is found it will throw an error.
    pub async fn perform_bulk_get(
        &self,
        job_ids: Vec<String>,
        customer_number: Option<String>,
    ) -> Result<BulkGet, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let payload = ReportsAction {
            action: "GET".to_string(),
            job_ids,
        };
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports",
                server,
                cn
            );
        let response = self
            .transporter
            .post::<ReportsAction>(uri, payload.clone())
            .await?;
        if response.status() == StatusCode::OK {
            let res_str = response_to_body(response).await?;
            let a: BulkGet = serde_json::from_str(res_str.as_str())?;
            return Ok(a);
            }
    }
    Err("No report found!".into())
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
        for server in &self.region_uri.servers {
            let uri = format!(
                "{}/rest/v1/{}/fax/reports",
                server,
                cn
            );
            let response = self
            .transporter
            .delete(uri)
            .await?;

            if response.status() == StatusCode::OK {
                let res_str = response_to_body(response).await?;
                let a: BulkDelete = serde_json::from_str(res_str.as_str())?;
                return Ok(a);
            }
        }
        Err("No report found!".into())
        
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
            transporter: Transporter::new(self.credentials),
            region_uri: determine_region_uri(self.region),
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
