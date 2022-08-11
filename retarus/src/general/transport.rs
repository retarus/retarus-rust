use std::{error::Error, time::Duration};

use crate::fax::responses::FaxJobResponse;
use crate::general::uri::RegionUri;
use hyper::{client::HttpConnector, Body, Client, Method, Request, Response, StatusCode};
use hyper_tls::HttpsConnector;
use serde::{de::DeserializeOwned, Serialize};
use tokio_compat_02::FutureExt;

use super::{creds::Credentials, job::Job};

/// Specifie the region and your login credentials.
#[derive(Debug, Clone)]
pub struct Transporter {
    /// Which server should be used to send the fax to.
    region_uri: RegionUri,
    credentials: Credentials,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Transporter {
    /// Creates a new Transporter that contains the endpoint_uri, credentails and a http client, that pools the connections and is able to communitcate with https endpoints.
    pub fn new(region_uri: RegionUri, credentials: Credentials) -> Transporter {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build::<_, hyper::Body>(https);
        Transporter {
            region_uri,
            credentials,
            client,
        }
    }

    // Send a job to the server. (Sends a fax to the specified endpoint server)
    pub async fn send_job(
        self,
        job: Job,
        customer_number: String,
    ) -> Result<FaxJobResponse, Box<dyn Error>> {
        let payload = serde_json::to_string(&job)?.as_bytes().to_owned();
        let uri = format!(
            "{}/rest/v1/{}/fax",
            self.region_uri.servers.get(0).unwrap(),
            customer_number);

        println!("uri: {:?}", uri);
        let req = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.encode()),
            )
            .body(Body::from(payload))?;
        let response = self.client.request(req).compat().await?;
        if response.status() == StatusCode::OK || response.status() == StatusCode::CREATED {
            let res_str = response_to_body(response).await?;
            let a: FaxJobResponse = serde_json::from_str(res_str.as_str())?;
            Ok(a)
        } else {
            let res_str = response_to_body(response).await?;
            Err(res_str.into())
        }
    }
    /// Transporter function to GET or DELETE report from a job id.
    pub async fn report_action<Y: DeserializeOwned + Serialize>(
        self,
        request_method: Method,
        customer_number: String,
        job_id: String,
    ) -> Result<Y, Box<dyn Error>> {
        let uri = format!(
            "{}/rest/v1/{}/fax/reports/{}",
            self.region_uri.servers.get(0).unwrap(),
            customer_number,
            job_id
        );

        let req = Request::builder()
            .uri(uri)
            .method(request_method)
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.encode()),
            )
            .header("Content-Type", "application/json")
            .body(Body::default())?;

        let response = self.client.request(req).await?;
        if response.status() == StatusCode::OK {
            let res_str = response_to_body(response).await?;
            let a: Y = serde_json::from_str(res_str.as_str())?;
            return Ok(a);
        } else if response.status() == StatusCode::NOT_FOUND {
            Err("No Fax report was found, please try again".into())
        } else {
            let res_str = response_to_body(response).await?;
            Err(res_str.into())
        }
    }

    /// Transporter function that handles bulk report requests GET, POST, DELETE for fax.
    /// Build and send the reuquest to the server, handles the response and returns the corresponding JobError for the response.
    pub async fn bulk_action<Y: DeserializeOwned + Serialize, T: DeserializeOwned + Serialize>(
        self,
        request_method: Method,
        customer_number: String,
        payload: Option<Y>,
    ) -> Result<T, Box<dyn Error>> {
        let uri = format!(
            "{}/rest/v1/{}/fax/reports",
            self.region_uri.servers.get(0).unwrap(),
            customer_number
        );

        let mut body = Body::default();

        if request_method == Method::POST {
            let payload = serde_json::to_string(&payload)?.as_bytes().to_owned();
            body = Body::from(payload);
        }
        let req = Request::builder()
            .uri(uri)
            .method(request_method)
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.encode()),
            )
            .header("Content-Type", "application/json")
            .body(body)?;

        let response = self.client.request(req).await?;
        println!("{:?}", response.status());
        if response.status() == StatusCode::OK {
            let res_str = response_to_body(response).await?;
            let a: T = serde_json::from_str(res_str.as_str())?;
            return Ok(a);
        }
        if response.status() == StatusCode::NOT_FOUND {
            Err("No report found!".into())
        } else {
            let res_str = response_to_body(response).await?;
            Err(res_str.into())
        }
    }
}

pub async fn response_to_body(resp: Response<Body>) -> Result<String, Box<dyn Error>> {
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let string = String::from_utf8(body_bytes.to_vec())?;
    Ok(string)
}

#[cfg(test)]
mod tests {
    use tokio;

    use crate::general::{
        creds::Credentials,
        document::Document,
        job::Job,
        uri::{Region, RegionUri},
    };

    use super::Transporter;

    #[tokio::test]
    async fn test_send_job() {
        let customer_number = "99999";
        let transporter = Transporter::new(
            RegionUri::new(
                Region::Custom("http://faxws.service.faxstaging.mucre1.retloc".to_string()),
                "",
                vec!["http://faxws.service.faxstaging.mucre1.retloc"],
            ),
            creds,
        );
        let doc = Document::new(
            "test.pdf".to_string(),
            std::fs::read("assets/testPdf.pdf").unwrap(),
            None,
        );
        let job = Job::builder()
            .add_recipient("+498900000000".to_string())
            .add_document(doc)
            .build();
        let response = transporter
            .send_job(job, customer_number.to_string())
            .await
            .unwrap();
        assert_ne!(response.job_id, String::new())
    }

    #[test]
    fn test_bulk_action() {}
}
