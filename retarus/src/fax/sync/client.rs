use std::error::Error;
use futures::executor::block_on;

use hyper::Method;
use tokio::runtime::Runtime;

use crate::{fax::{ responses::{DeletedReport, FaxJobResponse, BulkDelete, BulkGet}}, general::{transport::Transporter, job::Job, report::{Report, ReportsAction}, uri::{Region, determine_region_uri}, creds::Credentials}};



pub struct ClientSDK {
    transporter: Transporter,
    customer_number: String,
    runtime: Runtime
}
impl ClientSDK {
    pub fn builder() -> ClientSDKBuilder {
        ClientSDKBuilder::default()
    }

    /// Send a fax to the retarus server.
    pub fn send_job(&self, customer_number: Option<String>, job: Job) -> Result<FaxJobResponse, Box<dyn std::error::Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().send_job(job, cn);
        self.runtime.block_on(future)
    }

    pub fn get_fax_report(&self, job_id: String, customer_number: Option<String>) -> Result<Report, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().report_action::<Report>(Method::GET, cn, job_id);
        self.runtime.block_on(future)
    }

    pub fn delete_fax_report(&self, job_id: String, customer_number: Option<String>) -> Result<DeletedReport, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().report_action::<DeletedReport>(Method::DELETE, cn, job_id);
        self.runtime.block_on(future)
    }

    pub fn get_fax_reports(&self, customer_number: Option<String>) -> Result<BulkGet, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().bulk_action::<String, BulkGet>(Method::GET, cn, None);
        self.runtime.block_on(future)
    }

    pub fn perform_bulk_operation(&self, customer_number: Option<String>, payload: ReportsAction) -> Result<BulkDelete, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().bulk_action::<ReportsAction, BulkDelete>(Method::POST, cn, Some(payload));
        self.runtime.block_on(future)
    }

    pub fn delete_report_bulk(&self, customer_number: Option<String>) -> Result<BulkDelete, Box<dyn Error>> {
        let mut cn = self.customer_number.clone();
        if customer_number.is_some() {
            cn = customer_number.unwrap();
        }
        let future = self.transporter.clone().bulk_action::<String, BulkDelete>(Method::DELETE, cn, None);
        self.runtime.block_on(future)
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
            runtime: tokio::runtime::Runtime::new().unwrap()
        };
    }
}
