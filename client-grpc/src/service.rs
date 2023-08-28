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

    /// Returns a [`tonic::Response`] containing a [`ReadyResponse`](Self::ReadyResponse)
    /// Takes an [`ReadyRequest`](Self::ReadyRequest).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`Code::Unknown`](tonic::Code::Unknown) if
    /// the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_compliance_client_grpc::prelude::*;
    /// use tonic::transport::Channel;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let client = ComplianceClient::new_client(&host, port, "compliance");
    ///     let response = client
    ///         .is_ready(compliance::ReadyRequest {})
    ///         .await?;
    ///     println!("RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`FlightPlanResponse`](super::FlightPlanResponse)
    /// Takes an [`FlightPlanRequest`](super::FlightPlanRequest).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`Code::Unknown`](tonic::Code::Unknown) if
    /// the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_compliance_client_grpc::prelude::*;
    /// use tonic::transport::Channel;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let client = ComplianceClient::new_client(&host, port, "compliance");
    ///     let response = client
    ///         .submit_flight_plan(compliance::FlightPlanRequest {
    ///             flight_plan_id: "".to_string(),
    ///             data: "".to_string(),
    ///         })
    ///         .await?;
    ///     println!("submit_flight_plan RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn submit_flight_plan(
        &self,
        request: super::FlightPlanRequest,
    ) -> Result<tonic::Response<super::FlightPlanResponse>, tonic::Status>;

    /// Returns a [`tonic::Response`] containing a [`FlightReleaseResponse`](super::FlightReleaseResponse)
    /// Takes an [`FlightReleaseRequest`](super::FlightReleaseRequest).
    ///
    /// # Errors
    ///
    /// Returns [`tonic::Status`] with [`Code::Unknown`](tonic::Code::Unknown) if
    /// the server is not ready.
    ///
    /// # Examples
    /// ```
    /// use lib_common::grpc::get_endpoint_from_env;
    /// use svc_compliance_client_grpc::prelude::*;
    /// use tonic::transport::Channel;
    ///
    /// async fn example () -> Result<(), Box<dyn std::error::Error>> {
    ///     let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    ///     let client = ComplianceClient::new_client(&host, port, "compliance");
    ///     let response = client
    ///         .request_flight_release(compliance::FlightReleaseRequest {
    ///             flight_plan_id: "".to_string(),
    ///             data: "".to_string(),
    ///         })
    ///         .await?;
    ///     println!("submit_flight_plan RESPONSE={:?}", response.into_inner());
    ///     Ok(())
    /// }
    /// ```
    async fn request_flight_release(
        &self,
        request: super::FlightReleaseRequest,
    ) -> Result<tonic::Response<super::FlightReleaseResponse>, tonic::Status>;
}
