use std::{error::Error, fmt};

use serde::{Deserialize, Serialize};

use super::document::Document;


#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", rename = "faxJobReport")]
pub struct ReportResponse {
    #[serde(rename = "$value")]
    pub reports: Vec<SoapReport>
}


#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", rename = "faxJobReport")]
pub struct SoapReport {
    pub job_id: String,
    #[serde(rename = "$value")]
    pub fax_recipient: Vec<RecipientStatus>,
    pub document: Document,
    pub options: FaxOptions,
    pub pages: u16,
}


#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase", rename = "faxJobReport")]
pub struct Report {
    pub job_id: String,
    pub recipient_status: Vec<Option<RecipientStatus>>,
    pub pages: u16,
    pub reference: Reference,
}
#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Reference{
    pub customer_defined_id: String,
    pub billing_code: Option<String>,
    pub billing_info: Option<String>
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RecipientStatus {
    pub number: String,
    pub alternative_number: Option<String>,
    pub properties: Option<String>,
    pub status: String,
    pub reason: String,
    pub send_ts: Option<String>,
    pub duration_in_secs: u16,
    pub sent_to_number: Option<String>,
    pub remote_csid: Option<String>
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ReportsAction {
    pub action: String,
    pub job_ids: Vec<String>
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct FaxOptions {
    resolution: String,
    csid: String,
    header: String,
    is_blacklist_enabled: bool,
    is_express: bool,
    overlay: Overlay,
    coverpage_template_name: String,
    scheduleTS: String,
    job_expirey_minutes: String,
    job_reference: String
}

#[derive(Debug, Deserialize, Clone, Serialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Overlay {
    name: String,
    mode: String
}