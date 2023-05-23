//! Client Library: Client Functions, Structs, Traits
/// gRPC object traits to provide wrappers for grpc functions
#[tonic::async_trait]
pub trait Client<T>
where
    Self: Sized + lib_common::grpc::Client<T> + lib_common::grpc::ClientConnect<T>,
    T: Send + Clone,
{
    /// The type expected for ReadyRequest structs.
    type ReadyRequest;
    /// The type expected for ReadyResponse structs.
    type ReadyResponse;

    /// Wrapper for is_ready function.
    async fn is_ready(
        &self,
        request: tonic::Request<Self::ReadyRequest>,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status>;

    /// Wrapper for submit_flight_plan function
    async fn submit_flight_plan(
        &self,
        request: tonic::Request<super::FlightPlanRequest>,
    ) -> Result<tonic::Response<super::FlightPlanResponse>, tonic::Status>;

    /// Wrapper for request_flight_release function
    async fn request_flight_release(
        &self,
        request: tonic::Request<super::FlightReleaseRequest>,
    ) -> Result<tonic::Response<super::FlightReleaseResponse>, tonic::Status>;

    /// Wrapper for request_waypoints function
    async fn request_waypoints(
        &self,
        request: tonic::Request<super::WaypointsRequest>,
    ) -> Result<tonic::Response<super::WaypointsResponse>, tonic::Status>;

    /// Wrapper for request_restrictions function
    async fn request_restrictions(
        &self,
        request: tonic::Request<super::RestrictionsRequest>,
    ) -> Result<tonic::Response<super::RestrictionsResponse>, tonic::Status>;
}
