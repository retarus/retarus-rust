use std::{fmt};

use hyper::StatusCode;


#[derive(Debug, Clone)]
pub enum JobError {
    BadRequest,
    NotFound,
    Conflict,
    InternalServerError,
    UnkownError,
    NotImplemented
}
impl fmt::Display for JobError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            JobError::BadRequest => write!(f, "Client authorization is missing"),
            JobError::NotFound => write!(f, "No job report available for the given jobId; no recipient report available for the given jobId"),
            JobError::Conflict => write!(f, "Duplicate job"),
            JobError::InternalServerError => write!(f, "Cannot accept job, cannot query jobReport, cannot list jobs, cannot query recipient report, cannot apply transliteration in the send job"),
            JobError::UnkownError => write!(f, "Server signals that there was an unklnown problem, most likely with the backend adaptor"),
            JobError::NotImplemented => write!(f, "This Method is not available for this specified endpoint")
        }
    }
}
impl JobError {
    pub fn match_status_code(code: StatusCode) -> JobError {
        match code {
            StatusCode::BAD_REQUEST => JobError::BadRequest,
           _ => JobError::UnkownError 
        }
    }
}

impl std::error::Error for JobError {}
