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
pub struct JobResponse {
    pub job_id: String
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
impl SmsJob {
    // Get a builder instance to configure a sms job.
    pub fn builder() -> SmsJobBuilder {
        return SmsJobBuilder {
            options: None,
            messages: Vec::new(),
        }
    }
}

pub struct SmsJobBuilder {
    options: Option<Options>,
    messages: Vec<Message>
}
impl SmsJobBuilder {
    pub fn add_message(mut self, message: String, dst: Vec<String>) -> SmsJobBuilder {
        if self.messages.len() >= 3 {
            assert!(false, "Too many messages, a job can only handle 3 messages.")
        }
        let mut respt = vec![];
        for number in dst.iter() {
            respt.push(Recipient{ dst: number.to_owned(), customer_ref: None, blackout_periods: None });
        }
        let msg = Message { text: message, recipients: respt };
        self.messages.push(msg);
        self
    }
    pub fn add_messages(mut self, mut messages: Vec<Message>) -> SmsJobBuilder{
        if self.messages.len() >= 3 || messages.len() >= 3 || messages.len() + self.messages.len() >= 3 {
            assert!(false, "Too many messages, a job can only handle 3 messages.")
        }
        self.messages.append(&mut messages);
        self
    }

    pub fn configure_sms(mut self, options: Options) -> SmsJobBuilder {
        self.options = Some(options);
        self
    }
    pub fn build(self) -> SmsJob {
        SmsJob{options: self.options, messages: self.messages}
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JobReport {
    pub job_id: String,
    pub src: Option<String>,
    pub encoding: Option<String>,
    pub billcode: Option<String>,
    pub status_requested: Option<bool>,
    pub flash: Option<bool>,
    pub validity_min: Option<i32>,
    pub customer_ref: Option<String>,
    pub qos: Option<String>,
    pub receipt_ts: Option<String>,
    pub finished_ts: Option<String>,
    pub recipient_ids: Option<Vec<String>>
}


pub struct SmsFilterBuilder {
    filter: SmsFilter
}
impl SmsFilterBuilder{
    fn default() -> SmsFilterBuilder{
        SmsFilterBuilder { filter: SmsFilter { job_ids_only: None, from_ts: None, to_ts: None, open: None, offset: None, limit: None } }
    }
    pub fn set_limit(mut self, limit: i64) -> Self{
        self.filter.limit = Some(limit);
        self
    }
    pub fn set_from_ts(mut self, ts: DateTime::<Utc>) -> Self {
        self.filter.from_ts = Some(ts);
        self
    }
    pub fn set_to_ts(mut self, ts: DateTime::<Utc>) -> Self {
        self.filter.to_ts = Some(ts);
        self
    }
    pub fn set_offset(mut self, offset: usize) -> Self {
        self.filter.offset = Some(offset);
        self
    }
    pub fn only_job_ids(mut self, only_ids: bool) -> Self {
        self.filter.job_ids_only = Some(only_ids);
        self
    }
    pub fn build(self) -> SmsFilter {
        return self.filter
    }


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
    pub fn builder() -> SmsFilterBuilder {
        SmsFilterBuilder::default()
        }

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