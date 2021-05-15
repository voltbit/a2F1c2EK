#![allow(unused)]
use warp::Filter;
extern crate pretty_env_logger;

pub async fn external_server() {
    log::info!("Starting external server");

    let api = filters::jobs();
    let routes = api.with(warp::log("info"));
    warp::serve(routes).run(([127, 0, 0, 1], 4030)).await;
}

mod filters {
    use super::handlers;
    use crate::models::{AvailableJobRequest, JobRequest, JobResult};
    use warp::Filter;

    /// Job API
    pub fn jobs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        create_job()
            .or(get_available_job())
            .or(get_job())
            .or(create_job_result())
            .or(get_job_dataset())
    }

    /// POST /job
    pub fn create_job() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("job")
            .and(warp::post())
            .and(json_new_job())
            .and_then(handlers::create_job)
    }

    /// GET /job/available
    pub fn get_available_job(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("job" / "available")
            .and(warp::get())
            .and(json_available_job())
            .and_then(handlers::get_available_job)
    }

    /// GET /job/jobID
    pub fn get_job() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("job" / String)
            .and(warp::get())
            .and_then(handlers::get_job)
    }

    /// POST /job/result
    pub fn create_job_result(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("job" / "result")
            .and(warp::post())
            .and(json_new_result())
            .and_then(handlers::create_job_result)
    }

    /// GET /job/dataset/jobID
    pub fn get_job_dataset(
    ) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("job" / "dataset" / String)
            .and(warp::get())
            .and_then(handlers::get_job_dataset)
    }

    fn json_new_job() -> impl Filter<Extract = (JobRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_new_result() -> impl Filter<Extract = (JobResult,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }

    fn json_available_job(
    ) -> impl Filter<Extract = (AvailableJobRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use crate::internal_api::*;
    use crate::models::{AvailableJobRequest, JobRequest, JobResult};
    use std::convert::Infallible;
    use warp::http::StatusCode;

    // >>> Improve the error handling of internal API requests

    pub async fn get_job(job_id: String) -> Result<impl warp::Reply, Infallible> {
        log::info!("Getting the job request for {}", job_id);
        internal_read_dataset_ingestion_job(job_id).await.unwrap();
        Ok(StatusCode::OK)
    }

    pub async fn create_job(job_request: JobRequest) -> Result<impl warp::Reply, Infallible> {
        log::info!("Got new job: {}", job_request.job_id);
        internal_create_dataset_ingestion_job(job_request)
            .await
            .unwrap();
        Ok(StatusCode::CREATED)
    }

    pub async fn create_job_result(job_result: JobResult) -> Result<impl warp::Reply, Infallible> {
        log::info!(
            "Got new job result for {}: {:?}",
            job_result.job_id,
            job_result.status
        );
        internal_create_dataset_ingestion_result(job_result)
            .await
            .unwrap();
        Ok(StatusCode::CREATED)
        // Ok(StatusCode::OK)
    }

    pub async fn get_job_dataset(job_id: String) -> Result<impl warp::Reply, Infallible> {
        log::info!("Getting dataset for job: {}", job_id);
        internal_read_job_dataset(job_id).await.unwrap();
        Ok(StatusCode::OK)
    }

    pub async fn get_available_job(
        available_job_request: AvailableJobRequest,
    ) -> Result<impl warp::Reply, Infallible> {
        log::info!("Getting available job");
        internal_read_available_job(available_job_request)
            .await
            .unwrap();
        Ok(StatusCode::OK)
    }
}
