/// FlightPlanRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanRequest {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// FlightPlanResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightPlanResponse {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(bool, tag = "2")]
    pub submitted: bool,
    /// Optional error or warning message
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
/// FlightReleaseRequest
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightReleaseRequest {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(string, tag = "2")]
    pub data: ::prost::alloc::string::String,
}
/// FlightReleaseResponse
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightReleaseResponse {
    /// Flight Plan Id
    #[prost(string, tag = "1")]
    pub flight_plan_id: ::prost::alloc::string::String,
    /// JSON data of the flight plan
    #[prost(bool, tag = "2")]
    pub released: bool,
    /// Optional error or warning message
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
/// Are you Ready?
///
/// No arguments
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryIsReady {}
/// I'm Ready
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// True if ready
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// Generated server implementations.
pub mod compliance_rpc_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with ComplianceRpcServer.
    #[async_trait]
    pub trait ComplianceRpc: Send + Sync + 'static {
        /// is ready heartbeat
        async fn is_ready(
            &self,
            request: tonic::Request<super::QueryIsReady>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status>;
        /// submit flight plan
        async fn submit_flight_plan(
            &self,
            request: tonic::Request<super::FlightPlanRequest>,
        ) -> Result<tonic::Response<super::FlightPlanResponse>, tonic::Status>;
        /// release flight plan
        async fn request_flight_release(
            &self,
            request: tonic::Request<super::FlightReleaseRequest>,
        ) -> Result<tonic::Response<super::FlightReleaseResponse>, tonic::Status>;
    }
    /// ComplianceRpc
    #[derive(Debug)]
    pub struct ComplianceRpcServer<T: ComplianceRpc> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: ComplianceRpc> ComplianceRpcServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for ComplianceRpcServer<T>
    where
        T: ComplianceRpc,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/grpc.ComplianceRpc/isReady" => {
                    #[allow(non_camel_case_types)]
                    struct isReadySvc<T: ComplianceRpc>(pub Arc<T>);
                    impl<
                        T: ComplianceRpc,
                    > tonic::server::UnaryService<super::QueryIsReady>
                    for isReadySvc<T> {
                        type Response = super::ReadyResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::QueryIsReady>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).is_ready(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = isReadySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.ComplianceRpc/submitFlightPlan" => {
                    #[allow(non_camel_case_types)]
                    struct submitFlightPlanSvc<T: ComplianceRpc>(pub Arc<T>);
                    impl<
                        T: ComplianceRpc,
                    > tonic::server::UnaryService<super::FlightPlanRequest>
                    for submitFlightPlanSvc<T> {
                        type Response = super::FlightPlanResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlightPlanRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).submit_flight_plan(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = submitFlightPlanSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/grpc.ComplianceRpc/requestFlightRelease" => {
                    #[allow(non_camel_case_types)]
                    struct requestFlightReleaseSvc<T: ComplianceRpc>(pub Arc<T>);
                    impl<
                        T: ComplianceRpc,
                    > tonic::server::UnaryService<super::FlightReleaseRequest>
                    for requestFlightReleaseSvc<T> {
                        type Response = super::FlightReleaseResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::FlightReleaseRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).request_flight_release(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = requestFlightReleaseSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: ComplianceRpc> Clone for ComplianceRpcServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: ComplianceRpc> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ComplianceRpc> tonic::server::NamedService for ComplianceRpcServer<T> {
        const NAME: &'static str = "grpc.ComplianceRpc";
    }
}
