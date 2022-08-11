use serde::{Serialize, Deserialize};

use crate::general::report::Report;




/// Represents the response that will be send by the server if it reutrns a 200 or 201. 
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FaxJobResponse{
    pub job_id: String
}
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BulkGet {
    pub reports: Vec<Report>
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Deserialize,Serialize)]
pub struct DeletedReport {
    pub job_id: String,
    pub deleted: bool,
    pub readon: Option<String>
}

#[serde(rename_all = "camelCase")]
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BulkDelete {
    pub reports: Vec<DeletedReport>
}
