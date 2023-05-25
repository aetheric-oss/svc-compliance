#![doc = include_str!("../README.md")]
pub mod service;
pub use client::*;
pub use lib_common::grpc::{Client, ClientConnect, GrpcClient};

use lib_common::log_macros;
use tonic::async_trait;
use tonic::transport::Channel;

pub mod client {
    //! Client Library: Client Functions, Structs, Traits
    #![allow(unused_qualifications)]
    include!("grpc.rs");

    use tonic::transport::Channel;

    pub use rpc_service_client::RpcServiceClient;
    cfg_if::cfg_if! {
        if #[cfg(feature = "test_util")] {
            use lib_common::grpc_mock_client;
            use svc_compliance::grpc::server::{RpcServiceServer, ServerImpl};
            grpc_mock_client!(RpcServiceClient, RpcServiceServer, ServerImpl);
            super::log_macros!("grpc", "app::client::mock_compliance");
        } else {
            use lib_common::grpc_client;
            grpc_client!(RpcServiceClient);
            super::log_macros!("grpc", "app::client::compliance");
        }
    }
}

#[cfg(not(feature = "mock_client"))]
#[async_trait]
impl service::Client<RpcServiceClient<Channel>> for GrpcClient<RpcServiceClient<Channel>> {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: tonic::Request<Self::ReadyRequest>,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_info!("(is_ready) {} client.", self.get_name());
        grpc_debug!("(is_ready) request: {:?}", request);
        self.get_client().await?.is_ready(request).await
    }

    async fn submit_flight_plan(
        &self,
        request: tonic::Request<FlightPlanRequest>,
    ) -> Result<tonic::Response<FlightPlanResponse>, tonic::Status> {
        grpc_warn!("(submit_flight_plan) {} client.", self.get_name());
        grpc_debug!("(submit_flight_plan) request: {:?}", request);
        self.get_client().await?.submit_flight_plan(request).await
    }

    async fn request_flight_release(
        &self,
        request: tonic::Request<FlightReleaseRequest>,
    ) -> Result<tonic::Response<FlightReleaseResponse>, tonic::Status> {
        grpc_warn!("(request_flight_release) {} client.", self.get_name());
        grpc_debug!("(request_flight_release) request: {:?}", request);
        self.get_client()
            .await?
            .request_flight_release(request)
            .await
    }

    async fn request_waypoints(
        &self,
        request: tonic::Request<WaypointsRequest>,
    ) -> Result<tonic::Response<WaypointsResponse>, tonic::Status> {
        grpc_warn!("(request_waypoints) {} client.", self.get_name());
        grpc_debug!("(request_waypoints) request: {:?}", request);
        self.get_client().await?.request_waypoints(request).await
    }

    async fn request_restrictions(
        &self,
        request: tonic::Request<RestrictionsRequest>,
    ) -> Result<tonic::Response<RestrictionsResponse>, tonic::Status> {
        grpc_warn!("(request_restrictions) {} client.", self.get_name());
        grpc_debug!("(request_restrictions) request: {:?}", request);
        self.get_client().await?.request_restrictions(request).await
    }
}

#[cfg(any(feature = "mock_client"))]
#[async_trait]
impl service::Client<RpcServiceClient<Channel>> for GrpcClient<RpcServiceClient<Channel>> {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: tonic::Request<Self::ReadyRequest>,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
        grpc_debug!("(is_ready MOCK) request: {:?}", request);
        Ok(tonic::Response::new(ReadyResponse { ready: true }))
    }
    async fn submit_flight_plan(
        &self,
        request: tonic::Request<FlightPlanRequest>,
    ) -> Result<tonic::Response<FlightPlanResponse>, tonic::Status> {
        grpc_warn!("(submit_flight_plan MOCK) {} client.", self.get_name());
        grpc_debug!("(submit_flight_plan MOCK) request: {:?}", request);
        let request = request.into_inner();
        Ok(tonic::Response::new(FlightPlanResponse {
            flight_plan_id: request.flight_plan_id,
            submitted: true,
            result: None,
        }))
    }

    async fn request_flight_release(
        &self,
        request: tonic::Request<FlightReleaseRequest>,
    ) -> Result<tonic::Response<FlightReleaseResponse>, tonic::Status> {
        grpc_warn!("(request_flight_release MOCK) {} client.", self.get_name());
        grpc_debug!("(request_flight_release MOCK) request: {:?}", request);
        let request = request.into_inner();
        Ok(tonic::Response::new(FlightReleaseResponse {
            flight_plan_id: request.flight_plan_id,
            released: true,
            result: None,
        }))
    }

    async fn request_waypoints(
        &self,
        request: tonic::Request<WaypointsRequest>,
    ) -> Result<tonic::Response<WaypointsResponse>, tonic::Status> {
        grpc_warn!("(request_waypoints MOCK) {} client.", self.get_name());
        grpc_debug!("(request_waypoints MOCK) request: {:?}", request);
        Ok(tonic::Response::new(WaypointsResponse {
            waypoints: vec![],
        }))
    }

    async fn request_restrictions(
        &self,
        request: tonic::Request<RestrictionsRequest>,
    ) -> Result<tonic::Response<RestrictionsResponse>, tonic::Status> {
        grpc_warn!("(request_restrictions MOCK) {} client.", self.get_name());
        grpc_debug!("(request_restrictions MOCK) request: {:?}", request);
        Ok(tonic::Response::new(RestrictionsResponse {
            restrictions: vec![],
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Client as ServiceClient;

    use super::*;
    use tonic::transport::Channel;

    #[tokio::test]
    async fn test_client_connect() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let connection = client.get_client().await;
        println!("{:?}", connection);
        assert!(connection.is_ok());
    }

    #[tokio::test]
    async fn test_client_is_ready_request() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);

        let result = client.is_ready(tonic::Request::new(ReadyRequest {})).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner().ready, true);
    }

    #[tokio::test]
    async fn test_client_submit_flight_plan() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);

        let result = client
            .submit_flight_plan(tonic::Request::new(FlightPlanRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            }))
            .await;

        assert!(result.is_ok());
        let result: FlightPlanResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        assert_eq!(result.submitted, true);
    }

    #[tokio::test]
    async fn test_grpc_request_flight_release() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);

        let result = client
            .request_flight_release(tonic::Request::new(FlightReleaseRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            }))
            .await;

        assert!(result.is_ok());
        let result: FlightReleaseResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        assert_eq!(result.released, true);
    }

    #[tokio::test]
    async fn test_grpc_request_waypoints() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let filter = CoordinateFilter {
            min: Some(Coordinate {
                latitude: 30.0,
                longitude: -105.0,
            }),
            max: Some(Coordinate {
                latitude: 35.0,
                longitude: -100.0,
            }),
        };
        let result = client
            .request_waypoints(tonic::Request::new(WaypointsRequest {
                filter: Some(filter),
            }))
            .await;

        assert!(result.is_ok());
        let result: WaypointsResponse = result.unwrap().into_inner();
        println!("{:?}", result);
    }

    #[tokio::test]
    async fn test_grpc_request_restrictions() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client: GrpcClient<RpcServiceClient<Channel>> =
            GrpcClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let filter = CoordinateFilter {
            min: Some(Coordinate {
                latitude: 30.0,
                longitude: -105.0,
            }),
            max: Some(Coordinate {
                latitude: 35.0,
                longitude: -100.0,
            }),
        };
        let result = client
            .request_restrictions(tonic::Request::new(RestrictionsRequest {
                filter: Some(filter),
            }))
            .await;

        assert!(result.is_ok());
        let result: RestrictionsResponse = result.unwrap().into_inner();
        println!("{:?}", result);
    }
}
