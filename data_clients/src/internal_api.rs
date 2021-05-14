// use tokio::sync::mpsc;
use joblib::joblib::job_manager_client::JobManagerClient;
use joblib::joblib::{
    AvailableIngestionJobRequest, AvailableIngestionJobResponse, DataProtocol,
    DatasetIngestionJobRequest, DatasetIngestionJobResponse, DatasetIngestionJobStatusRequest,
    DatasetIngestionJobStatusResponse, DatasetIngestionResultRequest,
    DatasetIngestionResultResponse, IngestionStatus, JobDatasetRequest, JobDatasetResponse,
};

// pub struct InternalClient {
//     JobManagerClient: client,
// }

// pub fn start_client() -> InternalClient {
// }

// pub fn init_connection() {
//     let mut client = JobManagerClient::connect("http://[::1]:22000").await?;
// }

pub async fn make_dataset_ingestion_job_request() -> Result<(), Box<dyn std::error::Error>> {
    // let (tx, mut rx) = mpsc::channel(32);
    let channel = tonic::transport::Channel::from_static("http://[::1]:22000")
        .connect()
        .await
        .expect("make_dataset_ingestion_job_request channel fail");
    let mut client = JobManagerClient::new(channel);
    let mut dij_req = DatasetIngestionJobRequest {
        job_id: format!("job_test"),
        user_id: format!("user_test"),
        urgency: false,
        protocol: 0,
        query: format!("query_test"),
    };
    dij_req.set_protocol(DataProtocol::S3);
    let request = tonic::Request::new(dij_req);
    let response = client.create_ingestion_job(request).await?.into_inner();
    println!("RESPONSE={:?}", response);
    Ok(())
}
