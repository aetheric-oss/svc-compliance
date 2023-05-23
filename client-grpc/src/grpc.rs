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
    /// Status result for submitted
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
    /// Status result for released
    #[prost(bool, tag = "2")]
    pub released: bool,
    /// Optional error or warning message
    #[prost(string, optional, tag = "3")]
    pub result: ::core::option::Option<::prost::alloc::string::String>,
}
/// Waypoint
/// See example: <https://opennav.com/waypoint/NL>
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Waypoint {
    /// Label
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// Latitude
    #[prost(double, tag = "2")]
    pub latitude: f64,
    /// Longitude
    #[prost(double, tag = "3")]
    pub longitude: f64,
}
/// Latitude and Longitude
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Coordinate {
    /// Latitude
    #[prost(double, tag = "1")]
    pub latitude: f64,
    /// Longitude
    #[prost(double, tag = "2")]
    pub longitude: f64,
}
/// CoordinateFilter
/// A rectangle defined by two coordinates
///   where the edges are aligned North-South and East-West
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CoordinateFilter {
    /// The Southeastern vertex
    #[prost(message, optional, tag = "1")]
    pub min: ::core::option::Option<Coordinate>,
    /// The Northwestern vertex
    #[prost(message, optional, tag = "2")]
    pub max: ::core::option::Option<Coordinate>,
}
/// WaypointsRequest Body
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaypointsRequest {
    /// Coordinates by which to filter waypoints
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<CoordinateFilter>,
}
/// WaypointsResponse Body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WaypointsResponse {
    /// Waypoints
    #[prost(message, repeated, tag = "1")]
    pub waypoints: ::prost::alloc::vec::Vec<Waypoint>,
}
/// RestrictionsRequest Body
/// Request temporary flight restrictions
#[derive(Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestrictionsRequest {
    /// Coordinates by which to filter Restrictions
    #[prost(message, optional, tag = "1")]
    pub filter: ::core::option::Option<CoordinateFilter>,
}
/// RestrictionsResponse Body
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RestrictionsResponse {
    /// List of flight restrictions in the area
    #[prost(message, repeated, tag = "1")]
    pub restrictions: ::prost::alloc::vec::Vec<FlightRestriction>,
}
/// FlightRestriction
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FlightRestriction {
    /// ID
    #[prost(string, tag = "1")]
    pub identifier: ::prost::alloc::string::String,
    /// Restriction region vertices
    #[prost(message, repeated, tag = "2")]
    pub vertices: ::prost::alloc::vec::Vec<Coordinate>,
    /// Altitude floor in meters
    #[prost(int32, tag = "3")]
    pub altitude_meters_min: i32,
    /// Altitude ceiling in meters
    #[prost(int32, tag = "4")]
    pub altitude_meters_max: i32,
    /// Start time
    #[prost(message, optional, tag = "5")]
    pub timestamp_start: ::core::option::Option<::prost_types::Timestamp>,
    /// End time
    #[prost(message, optional, tag = "6")]
    pub timestamp_end: ::core::option::Option<::prost_types::Timestamp>,
    /// Type
    #[prost(string, tag = "7")]
    pub restriction_type: ::prost::alloc::string::String,
    /// Reason
    #[prost(string, tag = "8")]
    pub reason: ::prost::alloc::string::String,
}
/// ReadyRequest body
///
/// No arguments
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyRequest {}
/// ReadyResponse body
/// Indicates if the service is ready for requests
#[derive(Eq, Copy)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadyResponse {
    /// True if ready
    #[prost(bool, tag = "1")]
    pub ready: bool,
}
/// Generated client implementations.
#[cfg(not(tarpaulin_include))]
pub mod rpc_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// RpcService
    #[derive(Debug, Clone)]
    pub struct RpcServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl RpcServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> RpcServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> RpcServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            RpcServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// is ready heartbeat
        pub async fn is_ready(
            &mut self,
            request: impl tonic::IntoRequest<super::ReadyRequest>,
        ) -> Result<tonic::Response<super::ReadyResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/grpc.RpcService/isReady");
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// submit flight plan
        pub async fn submit_flight_plan(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightPlanRequest>,
        ) -> Result<tonic::Response<super::FlightPlanResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.RpcService/submitFlightPlan",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// release flight plan
        pub async fn request_flight_release(
            &mut self,
            request: impl tonic::IntoRequest<super::FlightReleaseRequest>,
        ) -> Result<tonic::Response<super::FlightReleaseResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.RpcService/requestFlightRelease",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// request waypoints
        pub async fn request_waypoints(
            &mut self,
            request: impl tonic::IntoRequest<super::WaypointsRequest>,
        ) -> Result<tonic::Response<super::WaypointsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.RpcService/requestWaypoints",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        /// request flight restrictions
        pub async fn request_restrictions(
            &mut self,
            request: impl tonic::IntoRequest<super::RestrictionsRequest>,
        ) -> Result<tonic::Response<super::RestrictionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/grpc.RpcService/requestRestrictions",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
