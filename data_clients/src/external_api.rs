use warp::Filter;
extern crate pretty_env_logger;

pub async fn external_server() {
    log::debug!("Started external server");

    let api = filters::jobs();
    let routes = api.with(warp::log("info"));
    warp::serve(routes).run(([127, 0, 0, 1], 4030)).await;
}

mod filters {
    use warp::Filter;

    use super::{handlers, models::JobRequest};

    /// Job API
    pub fn jobs() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        job_create().or(job_get())
    }

    /// POST /job
    pub fn job_create() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone
    {
        warp::path!("job")
            .and(warp::post())
            .and(json_body())
            .and_then(handlers::create_job_request)
    }

    /// GET /status/jobID
    pub fn job_get() -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
        warp::path!("status" / String)
            .and(warp::get())
            .and_then(handlers::get_job_request)
    }

    fn json_body() -> impl Filter<Extract = (JobRequest,), Error = warp::Rejection> + Clone {
        warp::body::content_length_limit(1024 * 16).and(warp::body::json())
    }
}

mod handlers {
    use super::models::JobRequest;
    use std::convert::Infallible;
    use warp::http::StatusCode;

    pub async fn get_job_request(job_id: String) -> Result<impl warp::Reply, Infallible> {
        log::debug!("[REST] getting the request {}", job_id);
        Ok(StatusCode::OK)
    }

    pub async fn create_job_request(
        job_request: JobRequest,
    ) -> Result<impl warp::Reply, Infallible> {
        log::debug!("Got job request: {:?}", job_request);
        Ok(StatusCode::CREATED)
    }
}

mod models {
    use serde_derive::{Deserialize, Serialize};

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub struct JobRequest {
        pub job_id: String,
        pub user_id: String,
        pub urgency: JobUrgency,
        pub protocol: String,
        pub query: String,
    }

    #[derive(Debug, Deserialize, Serialize, Clone)]
    pub enum JobUrgency {
        Critical,
        High,
        Medium,
        Low,
    }
}
