#![allow(unused)]
use joblib::joblib::job_manager_server::{JobManager, JobManagerServer};
use joblib::joblib::{
    AvailableIngestionJobRequest, AvailableIngestionJobResponse, DataProtocol,
    DatasetIngestionJobRequest, DatasetIngestionJobResponse, DatasetIngestionJobStatusRequest,
    DatasetIngestionJobStatusResponse, DatasetIngestionResultRequest,
    DatasetIngestionResultResponse, IngestionStatus, JobDatasetRequest, JobDatasetResponse,
};
use std::net::SocketAddr;
use tonic::{transport::Server, Request, Response, Status};

use crate::job_db::*;

#[derive(Default)]
struct JobController {
    db: KJobDB,
}

#[tonic::async_trait]
impl JobManager for JobController {
    async fn create_ingestion_job(
        &self,
        request: Request<DatasetIngestionJobRequest>,
    ) -> Result<Response<DatasetIngestionJobResponse>, Status> {
        let mut r = DatasetIngestionJobResponse {};
        log::info!("Received create ingestion job request");
        let p = convert_protocol_type(request.get_ref().protocol());
        self.db.lock().await.insert_job(Job {
            job_id: request.get_ref().job_id.clone(),
            user_id: request.get_ref().user_id.clone(),
            proto: p,
            urgency: request.get_ref().urgency.clone(),
            query: request.get_ref().query.clone(),
        });
        log::info!("{:?}", self.db);
        Ok(Response::new(r))
    }

    async fn read_ingestion_job_status(
        &self,
        request: Request<DatasetIngestionJobStatusRequest>,
    ) -> Result<Response<DatasetIngestionJobStatusResponse>, Status> {
        let mut r = DatasetIngestionJobStatusResponse { job_status: 0 };
        r.set_job_status(IngestionStatus::Pending);
        log::info!("Received read ingestion job request");
        if let Some(job) = self
            .db
            .lock()
            .await
            .get_job(request.get_ref().job_id.clone())
        {
            log::info!("Response from DB: {:?}", job);
        }
        log::info!("{:?}", self.db);
        Ok(Response::new(r))
    }

    async fn create_ingestion_result(
        &self,
        request: Request<DatasetIngestionResultRequest>,
    ) -> Result<Response<DatasetIngestionResultResponse>, Status> {
        let mut r = DatasetIngestionResultResponse {};
        log::info!("Received create ingestion result request");
        let st = request.get_ref().status();
        self.db.lock().await.insert_result(JobResult {
            result_id: request.get_ref().result_id.clone(),
            job_id: request.get_ref().job_id.clone(),
            status: convert_job_status_type(st),
            dataset: request.get_ref().dataset.clone(),
        });
        log::info!("{:?}", self.db);
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
        log::info!("{:?}", self.db);
        Ok(Response::new(r))
    }

    async fn read_available_ingestion_job(
        &self,
        request: Request<AvailableIngestionJobRequest>,
    ) -> Result<Response<AvailableIngestionJobResponse>, Status> {
        let mut r = joblib::joblib::AvailableIngestionJobResponse {
            job_id: format!("job42"),
        };
        log::info!("Received read available job request");
        log::info!("{:?}", self.db);
        Ok(Response::new(r))
    }
}

pub async fn controller_server() -> Result<(), Box<dyn std::error::Error + Send + 'static>> {
    let addr: SocketAddr = "[::1]:22000".parse().unwrap();
    let mut controller = JobController::default();
    controller.db = build_sync_db();
    Server::builder()
        .add_service(JobManagerServer::new(controller))
        .serve(addr)
        .await
        .expect("Failed to start controller server");
    Ok(())
}

fn convert_job_status_type(comm: joblib::joblib::IngestionStatus) -> crate::job_db::JobStatus {
    match comm {
        joblib::joblib::IngestionStatus::Pending => crate::job_db::JobStatus::Pending,
        joblib::joblib::IngestionStatus::Failed => crate::job_db::JobStatus::Failure,
        joblib::joblib::IngestionStatus::Successful => crate::job_db::JobStatus::Success,
    }
}

fn convert_protocol_type(comm: joblib::joblib::DataProtocol) -> crate::job_db::DataProtocol {
    match comm {
        joblib::joblib::DataProtocol::Undefined => crate::job_db::DataProtocol::UNDEFINED,
        joblib::joblib::DataProtocol::Jdbc => crate::job_db::DataProtocol::JDBC,
        joblib::joblib::DataProtocol::Odbc => crate::job_db::DataProtocol::ODBC,
        joblib::joblib::DataProtocol::S3 => crate::job_db::DataProtocol::S3,
        joblib::joblib::DataProtocol::Looker => crate::job_db::DataProtocol::LOOKER,
    }
}
