use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobRequest {
    pub job_id: String,
    pub user_id: String,
    pub urgency: bool,
    pub data_protocol: DataProtocol,
    pub query: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct JobResult {
    pub result_id: String,
    pub job_id: String,
    pub status: JobStatus,
    pub dataset: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AvailableJobRequest {
    pub protocol: DataProtocol,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum JobStatus {
    Success,
    Failure,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum DataProtocol {
    JDBC,
    ODBC,
    S3,
    LOOKER,
}
