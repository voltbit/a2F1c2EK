use joblib::joblib::job_manager_client::JobManagerClient;
use joblib::joblib::{
    AvailableIngestionJobRequest, AvailableIngestionJobResponse, DataProtocol,
    DatasetIngestionJobRequest, DatasetIngestionJobResponse, DatasetIngestionJobStatusRequest,
    DatasetIngestionJobStatusResponse, DatasetIngestionResultRequest,
    DatasetIngestionResultResponse, IngestionStatus, JobDatasetRequest, JobDatasetResponse,
};

// trait InternalAPI {
//     fn new() -> Self;
//     fn create_dataset_ingestion_job(&self) -> Result<(), Box<dyn std::error::Error>>;
//     fn create_dataset_ingestion_result(&self) -> Result<(), Box<dyn std::error::Error>>;
//     fn get_dataset_ingestion_job_status(&self) -> Result<(), Box<dyn std::error::Error>>;
//     fn get_available_ingestion_job(&self) -> Result<(), Box<dyn std::error::Error>>;
//     fn get_job_dataset(&self) -> Result<(), Box<dyn std::error::Error>>;
// }

struct InternalClient {
    client: JobManagerClient<tonic::transport::Channel>,
}

impl InternalClient {
    async fn new() -> InternalClient {
        let ch = tonic::transport::Channel::from_static("http://[::1]:22000")
            .connect()
            .await
            .expect("make_dataset_ingestion_job_request channel fail");
        InternalClient {
            client: JobManagerClient::new(ch),
        }
    }

    async fn create_dataset_ingestion_job(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut dij_req = DatasetIngestionJobRequest {
            job_id: format!("job_test"),
            user_id: format!("user_test"),
            urgency: false,
            protocol: 0,
            query: format!("query_test"),
        };
        dij_req.set_protocol(DataProtocol::S3);
        let request = tonic::Request::new(dij_req);
        let response = self
            .client
            .create_ingestion_job(request)
            .await?
            .into_inner();
        println!("RESPONSE={:?}", response);
        Ok(())
    }
}
