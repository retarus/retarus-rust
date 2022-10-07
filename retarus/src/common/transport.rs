use std::{error::Error, time::Duration};

use futures::Future;
use hyper::{client::HttpConnector, Body, Client, Method, Request, Response};
use hyper_tls::HttpsConnector;
use serde::{Serialize};
use tokio_compat_02::FutureExt;

use super::creds::Credentials;

/// Specifies the region and your login credentials.
#[derive(Debug, Clone)]
pub struct Transporter {
    /// Which server should be used to send the fax to.
    credentials: Credentials,
    client: Client<HttpsConnector<HttpConnector>>,
}

impl Transporter {
    /// Creates a new Transporter that contains the endpoint_uri, credentails and a http client that pools the connections and is able to communitcate with https endpoints.
    pub fn new(credentials: Credentials) -> Transporter {
        let https = HttpsConnector::new();
        let client = Client::builder()
            .pool_idle_timeout(Duration::from_secs(60))
            .pool_max_idle_per_host(10)
            .build::<_, hyper::Body>(https);
        Transporter {
            credentials,
            client,
        }
    }

    pub async fn get(&self, uri: String) -> Result<Response<Body>, Box<dyn Error>> {
        let req = Request::builder()
            .uri(uri)
            .method(Method::GET)
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.clone().encode()),
            )
            .header("Content-Type", "application/json")
            .body(Body::default())?;
        let response = self.client.request(req).compat().await?;
        Ok(response)
    }

    pub async fn post<T: Serialize>(
        &self,
        uri: String,
        payload: T,
    ) -> Result<Response<Body>, Box<dyn Error>> {
        let payload = serde_json::to_string(&payload)?.as_bytes().to_owned();

        let req = Request::builder()
            .uri(uri)
            .method(Method::POST)
            .header("Content-Type", "application/json")
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.clone().encode()),
            )
            .body(Body::from(payload))?;
        let response = self.client.request(req).compat().await?;
        Ok(response)
    }
    pub async fn delete(&self, uri: String) -> Result<Response<Body>, Box<dyn Error>> {
        let req = Request::builder()
            .uri(uri)
            .method(Method::DELETE)
            .header(
                "Authorization",
                format!("Basic {}", self.credentials.clone().encode()),
            )
            .header("Content-Type", "application/json")
            .body(Body::default())?;
        let response = self.client.request(req).compat().await?;
        Ok(response)
    }
}

pub async fn response_to_body(resp: Response<Body>) -> Result<String, Box<dyn Error>> {
    let body_bytes = hyper::body::to_bytes(resp.into_body()).await?;
    let string = String::from_utf8(body_bytes.to_vec())?;
    Ok(string)
}




/// Takes a future in and blocks the current thread until the future completes,
/// used if your programm should run synchronously.
pub fn blocking<F: Future>(mut future: F) -> F::Output 
{
    let runtime = tokio::runtime::Runtime::new().unwrap();
    
    let res = runtime.block_on(future);
    return res
}
