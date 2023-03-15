use super::{document::Document};

/// This represents a fax job that will be transmitted  to the Retarus server to send a fax.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Job {
    /// A list of all numbers that should receive a fax.
    recipients: Vec<Number>,
    /// List of documents that should be send as fax to the specified numbers.
    documents: Vec<Document>,
}
impl Job {
    /// Send a fax to the server.
    /// # Examples
    /// ```rust
    /// use retarus::fax::document::Document;
    /// use retarus::fax::job::Job;
    /// use retarus::fax::job::Number;
    /// // first create a document
    /// let docs = vec![Document::new(String::from("important_document.pdf"), "abc_content".as_bytes().to_vec(), None )];
    /// // Then get a vec of all your number
    /// let numbers = vec!["491231255903".to_string()];
    /// // Create and send the job
    /// let response = Job::builder().add_recipients(numbers).add_documents(docs);
    /// ```
    fn new(numbers: Vec<Number>, documents: Vec<Document>) -> Job {
        Job {
            recipients: numbers,
            documents,
        }
    }
    pub fn builder() -> JobBuilder {
        JobBuilder::default()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Number {
    number: String,
}

/// Build a Job using this builder. Use defaults specified in the Fax4App SDK.
/// # Examples
///
/// ```rust
/// use retarus::fax::job::JobBuilder;
/// let numbers = vec!["+498900000000".to_string(), "+48090000000".to_string()];
/// let job = JobBuilder::default()
///             .add_recipients(numbers)
///             .build();
/// ```
pub struct JobBuilder {
    recipients: Vec<Number>,
    documents: Vec<Document>,
}
impl JobBuilder {
    /// Create a JobBuilder instance
    pub fn default() -> JobBuilder {
        JobBuilder {
            recipients: Vec::new(),
            documents: Vec::new(),
        }
    }

    /// Add a vector of recipients to the job (recipient number)
    pub fn add_recipients(mut self, recipients: Vec<String>) -> JobBuilder {
        for recipient in recipients.iter() {
            self.recipients.push(Number {
                number: recipient.to_owned(),
            })
        }
        self
    }

    /// To add a single phone number to the job.
    pub fn add_recipient(mut self, recipient: String) -> JobBuilder {
        self.recipients.push(Number { number: recipient });
        self
    }

    pub fn add_document(mut self, doc: Document) -> JobBuilder {
        self.documents.push(doc);
        self
    }

    pub fn add_documents(mut self, docs: Vec<Document>) -> JobBuilder {
        self.documents = docs;
        self
    }
    /// Build a job from the given arguments.
    pub fn build(self) -> Job {
        Job {
            recipients: self.recipients,
            documents: self.documents,
        }
    }
}


#[test]
fn test_job_builder() {
    let number = "+490000000000";
    let numbers = vec!["+499000000000".to_string(), "+49800000000000".to_string()];
    let job = JobBuilder::default()
    .add_recipient(number.to_string())
    .add_recipients(numbers)
    .build();
    assert_eq!(job.recipients.len(), 3)
}