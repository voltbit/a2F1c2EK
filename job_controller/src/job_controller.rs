use joblib::joblib::job_manager_server::{JobManager, JobManagerServer};
use joblib::joblib::{
    AvailableIngestionJobRequest, AvailableIngestionJobResponse, DataProtocol,
    DatasetIngestionJobRequest, DatasetIngestionJobResponse, DatasetIngestionJobStatusRequest,
    DatasetIngestionJobStatusResponse, DatasetIngestionResultRequest,
    DatasetIngestionResultResponse, IngestionStatus, JobDatasetRequest, JobDatasetResponse,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

#[derive(Default)]
struct JobController;

#[tonic::async_trait]
impl JobManager for JobController {
    async fn create_ingestion_job(
        &self,
        request: Request<DatasetIngestionJobRequest>,
    ) -> Result<Response<DatasetIngestionJobResponse>, Status> {
        let mut r = DatasetIngestionJobResponse {};
        log::info!("Received create ingestion job request");
        Ok(Response::new(r))
    }

    async fn read_ingestion_job_status(
        &self,
        request: Request<DatasetIngestionJobStatusRequest>,
    ) -> Result<Response<DatasetIngestionJobStatusResponse>, Status> {
        let mut r = DatasetIngestionJobStatusResponse { job_status: 0 };
        r.set_job_status(IngestionStatus::Pending);
        log::info!("Received read ingestion job request");
        Ok(Response::new(r))
    }

    async fn create_ingestion_result(
        &self,
        request: Request<DatasetIngestionResultRequest>,
    ) -> Result<Response<DatasetIngestionResultResponse>, Status> {
        let mut r = DatasetIngestionResultResponse {};
        log::info!("Received create ingestion result request");
        Ok(Response::new(r))
    }

    async fn read_job_dataset(
        &self,
        request: Request<JobDatasetRequest>,
    ) -> Result<Response<JobDatasetResponse>, Status> {
        let mut r = JobDatasetResponse {
            dataset: format!("test_dataset"),
        };
        log::info!("Received read dataset request");
        Ok(Response::new(r))
    }

    async fn read_available_ingestion_job(
        &self,
        request: Request<AvailableIngestionJobRequest>,
    ) -> Result<Response<AvailableIngestionJobResponse>, Status> {
        let mut r = joblib::joblib::AvailableIngestionJobResponse {
            job_id: format!("job42"),
        };
        log::info!("Received read dataset request");
        Ok(Response::new(r))
    }
}

pub async fn controller_server() -> Result<(), Box<dyn std::error::Error + Send + 'static>> {
    let addr: SocketAddr = "[::1]:22000".parse().unwrap();
    let controller = JobController::default();
    Server::builder()
        .add_service(JobManagerServer::new(controller))
        .serve(addr)
        .await
        .expect("Failed to start controller server");
    Ok(())
}
