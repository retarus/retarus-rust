use chrono::{DateTime, Utc};


#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    src: Option<String>,
    encoding: Option<String>,
    billcode: Option<String>,
    status_requested: Option<bool>,
    flash: Option<bool>,
    customer_ref: Option<String>,
    validity_min: Option<i32>,
    max_parts: Option<i32>,
    invalid_characters: Option<String>,
    qos: Option<String>,
    job_period: Option<String>,
    duplicate_detection: Option<bool>,
    blackout_periods: Option<Vec<String>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Recipient {
    dst: String,
    customer_ref: Option<String>,
    blackout_periods: Option<String>,
}
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    text: String,
    recipients: Vec<Recipient>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmsJob {
    options: Option<Options>,
    messages: Vec<Message>
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReport {
    job_id: String,
    src: String,
    encoding: String,
    billcode: String,
    status_requested: bool,
    flash: bool,
    validity_min: i32,
    customer_ref: String,
    qos: String,
    receipt_ts: String,
    finished_ts: String,
    recipient_ids: Vec<String>
}

pub struct SmsFilter {
    job_ids_only: Option<bool>,
    from_ts: Option<DateTime::<Utc>>,
    to_ts: Option<DateTime::<Utc>>,
    open: Option<bool>,
    offset: Option<usize>,
    limit: Option<i64>,
}
impl SmsFilter {
    pub fn create_filter_string(&self) -> String{
        let mut query = String::from("?");

        if &true == &self.job_ids_only.is_some() {
            query = format!("{}jobIdsOnly={}&", query,  &self.job_ids_only.unwrap())
        }
        if &true == &self.from_ts.is_some() {
            query = format!("{}fromTs={}&", query,  &self.from_ts.unwrap())
        }
        if &true == &self.to_ts.is_some() {
            query = format!("{}toTs={}&", query,  &self.to_ts.unwrap())
        }
        if &true == &self.open.is_some() {
            query = format!("{}open={}&", query,  &self.open.unwrap())
        }
        if &true == &self.offset.is_some() {
            query = format!("{}offset={}&", query,  &self.offset.unwrap())
        }
        if &true == &self.limit.is_some() {
            query = format!("{}limit={}&", query,  &self.limit.unwrap())
        }
        return query;
    }
}