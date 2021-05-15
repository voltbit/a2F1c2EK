#![allow(unused)]
use joblib::joblib::job_manager_client::JobManagerClient;
use joblib::joblib::{
    AvailableIngestionJobRequest, AvailableIngestionJobResponse, DataProtocol,
    DatasetIngestionJobRequest, DatasetIngestionJobResponse, DatasetIngestionJobStatusRequest,
    DatasetIngestionJobStatusResponse, DatasetIngestionResultRequest,
    DatasetIngestionResultResponse, IngestionStatus, JobDatasetRequest, JobDatasetResponse,
};

use crate::models::{AvailableJobRequest, JobRequest, JobResult};

pub async fn make_client() -> JobManagerClient<tonic::transport::Channel> {
    let channel = tonic::transport::Channel::from_static("http://[::1]:22000")
        .connect()
        .await
        .expect("make channel to job controller failed");
    JobManagerClient::new(channel)
}

pub async fn internal_create_dataset_ingestion_job(
    request: JobRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Inside internal API for job creation");
    let mut dij_req = DatasetIngestionJobRequest {
        job_id: request.job_id,
        user_id: request.user_id,
        urgency: request.urgency,
        protocol: 0,
        query: request.query,
    };
    dij_req.set_protocol(DataProtocol::S3);
    let request = tonic::Request::new(dij_req);
    let mut client = make_client().await;
    let response: DatasetIngestionJobResponse =
        client.create_ingestion_job(request).await?.into_inner();
    log::info!("Response: {:?}", response);
    Ok(())
}

pub async fn internal_create_dataset_ingestion_result(
    result: JobResult,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("Inside internal API for result creation");
    let mut dir_req = DatasetIngestionResultRequest {
        result_id: result.result_id,
        job_id: result.job_id,
        status: 0,
        dataset: result.dataset,
    };
    let request = tonic::Request::new(dir_req);
    let mut client = make_client().await;
    let response: DatasetIngestionResultResponse =
        client.create_ingestion_result(request).await?.into_inner();
    log::info!("Response: {:?}", response);
    Ok(())
}

pub async fn internal_read_dataset_ingestion_job(
    job_id: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut dijs_req = DatasetIngestionJobStatusRequest { job_id };
    let request = tonic::Request::new(dijs_req);
    let mut client = make_client().await;
    let response: DatasetIngestionJobStatusResponse = client
        .read_ingestion_job_status(request)
        .await?
        .into_inner();
    log::info!("Response: {:?}", response);
    Ok(())
}

pub async fn internal_read_available_job(
    request: AvailableJobRequest,
) -> Result<(), Box<dyn std::error::Error>> {
    log::info!("I am in the internal method");
    let mut aij_req = AvailableIngestionJobRequest { protocol: 0 };
    let request = tonic::Request::new(aij_req);
    let mut client = make_client().await;
    let response: AvailableIngestionJobResponse = client
        .read_available_ingestion_job(request)
        .await?
        .into_inner();
    log::info!("Response: {:?}", response);
    Ok(())
}

pub async fn internal_read_job_dataset(job_id: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut jd_req = JobDatasetRequest { job_id };
    let request = tonic::Request::new(jd_req);
    let mut client = make_client().await;
    let response: JobDatasetResponse = client.read_job_dataset(request).await?.into_inner();
    log::info!("Response: {:?}", response);
    Ok(())
}
