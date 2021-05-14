//
//The comments are added specifically to make it easier to follow along with the
//interview task requirements - hence the source detail.
//
//All responses are relative to the corresponding request and are made by the
//Controller.

/// Source: user facing app
/// Defines a new ingestion job
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionJobRequest {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub user_id: ::prost::alloc::string::String,
    #[prost(bool, tag = "3")]
    pub urgency: bool,
    #[prost(enumeration = "DataProtocol", tag = "4")]
    pub protocol: i32,
    #[prost(string, tag = "5")]
    pub query: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionJobResponse {}
/// Source: user facing app
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionJobStatusRequest {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionJobStatusResponse {
    #[prost(enumeration = "IngestionStatus", tag = "1")]
    pub job_status: i32,
}
/// Source: dataset ingestor
/// Defines the result of an ingestion job
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionResultRequest {
    #[prost(string, tag = "1")]
    pub result_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub job_id: ::prost::alloc::string::String,
    #[prost(enumeration = "IngestionStatus", tag = "3")]
    pub status: i32,
    #[prost(string, tag = "4")]
    pub dataset: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DatasetIngestionResultResponse {}
/// Source: dataset ingestor
/// Defines the request an igestor makes to get a suitable job
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableIngestionJobRequest {
    #[prost(enumeration = "DataProtocol", tag = "1")]
    pub protocol: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AvailableIngestionJobResponse {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
/// Source: insight generator
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobDatasetRequest {
    #[prost(string, tag = "1")]
    pub job_id: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct JobDatasetResponse {
    #[prost(string, tag = "1")]
    pub dataset: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DataProtocol {
    Undefined = 0,
    Jdbc = 1,
    Odbc = 2,
    S3 = 3,
    Looker = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IngestionStatus {
    Pending = 0,
    Successful = 1,
    Failed = 2,
}
#[doc = r" Generated client implementations."]
pub mod job_manager_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct JobManagerClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl JobManagerClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> JobManagerClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn create_ingestion_job(
            &mut self,
            request: impl tonic::IntoRequest<super::DatasetIngestionJobRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionJobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/joblib.JobManager/CreateIngestionJob");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn create_ingestion_result(
            &mut self,
            request: impl tonic::IntoRequest<super::DatasetIngestionResultRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionResultResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/joblib.JobManager/CreateIngestionResult");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_ingestion_job_status(
            &mut self,
            request: impl tonic::IntoRequest<super::DatasetIngestionJobStatusRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionJobStatusResponse>, tonic::Status>
        {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/joblib.JobManager/ReadIngestionJobStatus");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_available_ingestion_job(
            &mut self,
            request: impl tonic::IntoRequest<super::AvailableIngestionJobRequest>,
        ) -> Result<tonic::Response<super::AvailableIngestionJobResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/joblib.JobManager/ReadAvailableIngestionJob",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn read_job_dataset(
            &mut self,
            request: impl tonic::IntoRequest<super::JobDatasetRequest>,
        ) -> Result<tonic::Response<super::JobDatasetResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/joblib.JobManager/ReadJobDataset");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for JobManagerClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
    impl<T> std::fmt::Debug for JobManagerClient<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "JobManagerClient {{ ... }}")
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod job_manager_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with JobManagerServer."]
    #[async_trait]
    pub trait JobManager: Send + Sync + 'static {
        async fn create_ingestion_job(
            &self,
            request: tonic::Request<super::DatasetIngestionJobRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionJobResponse>, tonic::Status>;
        async fn create_ingestion_result(
            &self,
            request: tonic::Request<super::DatasetIngestionResultRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionResultResponse>, tonic::Status>;
        async fn read_ingestion_job_status(
            &self,
            request: tonic::Request<super::DatasetIngestionJobStatusRequest>,
        ) -> Result<tonic::Response<super::DatasetIngestionJobStatusResponse>, tonic::Status>;
        async fn read_available_ingestion_job(
            &self,
            request: tonic::Request<super::AvailableIngestionJobRequest>,
        ) -> Result<tonic::Response<super::AvailableIngestionJobResponse>, tonic::Status>;
        async fn read_job_dataset(
            &self,
            request: tonic::Request<super::JobDatasetRequest>,
        ) -> Result<tonic::Response<super::JobDatasetResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct JobManagerServer<T: JobManager> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: JobManager> JobManagerServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T, B> Service<http::Request<B>> for JobManagerServer<T>
    where
        T: JobManager,
        B: HttpBody + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/joblib.JobManager/CreateIngestionJob" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIngestionJobSvc<T: JobManager>(pub Arc<T>);
                    impl<T: JobManager>
                        tonic::server::UnaryService<super::DatasetIngestionJobRequest>
                        for CreateIngestionJobSvc<T>
                    {
                        type Response = super::DatasetIngestionJobResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatasetIngestionJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_ingestion_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateIngestionJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/joblib.JobManager/CreateIngestionResult" => {
                    #[allow(non_camel_case_types)]
                    struct CreateIngestionResultSvc<T: JobManager>(pub Arc<T>);
                    impl<T: JobManager>
                        tonic::server::UnaryService<super::DatasetIngestionResultRequest>
                        for CreateIngestionResultSvc<T>
                    {
                        type Response = super::DatasetIngestionResultResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatasetIngestionResultRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).create_ingestion_result(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = CreateIngestionResultSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/joblib.JobManager/ReadIngestionJobStatus" => {
                    #[allow(non_camel_case_types)]
                    struct ReadIngestionJobStatusSvc<T: JobManager>(pub Arc<T>);
                    impl<T: JobManager>
                        tonic::server::UnaryService<super::DatasetIngestionJobStatusRequest>
                        for ReadIngestionJobStatusSvc<T>
                    {
                        type Response = super::DatasetIngestionJobStatusResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DatasetIngestionJobStatusRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).read_ingestion_job_status(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReadIngestionJobStatusSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/joblib.JobManager/ReadAvailableIngestionJob" => {
                    #[allow(non_camel_case_types)]
                    struct ReadAvailableIngestionJobSvc<T: JobManager>(pub Arc<T>);
                    impl<T: JobManager>
                        tonic::server::UnaryService<super::AvailableIngestionJobRequest>
                        for ReadAvailableIngestionJobSvc<T>
                    {
                        type Response = super::AvailableIngestionJobResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AvailableIngestionJobRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut =
                                async move { (*inner).read_available_ingestion_job(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReadAvailableIngestionJobSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/joblib.JobManager/ReadJobDataset" => {
                    #[allow(non_camel_case_types)]
                    struct ReadJobDatasetSvc<T: JobManager>(pub Arc<T>);
                    impl<T: JobManager> tonic::server::UnaryService<super::JobDatasetRequest> for ReadJobDatasetSvc<T> {
                        type Response = super::JobDatasetResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::JobDatasetRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).read_job_dataset(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = ReadJobDatasetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: JobManager> Clone for JobManagerServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: JobManager> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: JobManager> tonic::transport::NamedService for JobManagerServer<T> {
        const NAME: &'static str = "joblib.JobManager";
    }
}
