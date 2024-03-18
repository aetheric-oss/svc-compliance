//! Client Library: Client Functions, Structs, Traits
#![allow(unused_qualifications)]
include!("grpc.rs");

use super::*;

#[cfg(any(not(feature = "stub_client"), feature = "stub_backends"))]
use lib_common::grpc::ClientConnect;
use lib_common::grpc::{Client, GrpcClient};
use rpc_service_client::RpcServiceClient;
/// GrpcClient implementation of the RpcServiceClient
pub type ComplianceClient = GrpcClient<RpcServiceClient<Channel>>;

cfg_if::cfg_if! {
    if #[cfg(feature = "stub_backends")] {
        use svc_compliance::grpc::server::{RpcServiceServer, ServerImpl};

        #[tonic::async_trait]
        impl lib_common::grpc::ClientConnect<RpcServiceClient<Channel>> for ComplianceClient {
            /// Get a connected client object
            async fn connect(
                &self,
            ) -> Result<RpcServiceClient<Channel>, tonic::transport::Error> {
                let (client, server) = tokio::io::duplex(1024);

                let region = Box::<svc_compliance::region::RegionImpl>::default();

                let grpc_service = ServerImpl {
                    mq_channel: None,
                    region,
                };

                lib_common::grpc::mock::start_mock_server(
                    server,
                    RpcServiceServer::new(grpc_service),
                )
                .await?;

                // Move client to an option so we can _move_ the inner value
                // on the first attempt to connect. All other attempts will fail.
                let mut client = Some(client);
                let channel = tonic::transport::Endpoint::try_from("http://[::]:50051")?
                    .connect_with_connector(tower::service_fn(move |_: tonic::transport::Uri| {
                        let client = client.take();

                        async move {
                            if let Some(client) = client {
                                Ok(client)
                            } else {
                                Err(std::io::Error::new(
                                    std::io::ErrorKind::Other,
                                    "Client already taken",
                                ))
                            }
                        }
                    }))
                    .await?;

                Ok(RpcServiceClient::new(channel))
            }
        }

        super::log_macros!("grpc", "app::client::mock::compliance");
    } else {
        lib_common::grpc_client!(RpcServiceClient);
        super::log_macros!("grpc", "app::client::compliance");
    }
}

#[cfg(not(feature = "stub_client"))]
#[async_trait]
impl service::Client<RpcServiceClient<Channel>> for ComplianceClient {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_info!("(is_ready) {} client.", self.get_name());
        grpc_debug!("(is_ready) request: {:?}", request);
        self.get_client().await?.is_ready(request).await
    }

    async fn submit_flight_plan(
        &self,
        request: FlightPlanRequest,
    ) -> Result<tonic::Response<FlightPlanResponse>, tonic::Status> {
        grpc_warn!("(submit_flight_plan) {} client.", self.get_name());
        grpc_debug!("(submit_flight_plan) request: {:?}", request);
        self.get_client().await?.submit_flight_plan(request).await
    }

    async fn request_flight_release(
        &self,
        request: FlightReleaseRequest,
    ) -> Result<tonic::Response<FlightReleaseResponse>, tonic::Status> {
        grpc_warn!("(request_flight_release) {} client.", self.get_name());
        grpc_debug!("(request_flight_release) request: {:?}", request);
        self.get_client()
            .await?
            .request_flight_release(request)
            .await
    }
}

#[cfg(feature = "stub_client")]
#[async_trait]
impl service::Client<RpcServiceClient<Channel>> for ComplianceClient {
    type ReadyRequest = ReadyRequest;
    type ReadyResponse = ReadyResponse;

    async fn is_ready(
        &self,
        request: Self::ReadyRequest,
    ) -> Result<tonic::Response<Self::ReadyResponse>, tonic::Status> {
        grpc_warn!("(is_ready MOCK) {} client.", self.get_name());
        grpc_debug!("(is_ready MOCK) request: {:?}", request);
        Ok(tonic::Response::new(ReadyResponse { ready: true }))
    }
    async fn submit_flight_plan(
        &self,
        request: FlightPlanRequest,
    ) -> Result<tonic::Response<FlightPlanResponse>, tonic::Status> {
        grpc_warn!("(submit_flight_plan MOCK) {} client.", self.get_name());
        grpc_debug!("(submit_flight_plan MOCK) request: {:?}", request);
        Ok(tonic::Response::new(FlightPlanResponse {
            flight_plan_id: request.flight_plan_id,
            submitted: true,
            result: None,
        }))
    }

    async fn request_flight_release(
        &self,
        request: FlightReleaseRequest,
    ) -> Result<tonic::Response<FlightReleaseResponse>, tonic::Status> {
        grpc_warn!("(request_flight_release MOCK) {} client.", self.get_name());
        grpc_debug!("(request_flight_release MOCK) request: {:?}", request);
        Ok(tonic::Response::new(FlightReleaseResponse {
            flight_plan_id: request.flight_plan_id,
            released: true,
            result: None,
        }))
    }
}

#[cfg(test)]
mod tests {
    use crate::service::Client as ServiceClient;
    // use lib_common::grpc::ClientConnect;

    use super::*;

    #[tokio::test]
    #[cfg(not(feature = "stub_client"))]
    async fn test_client_connect() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client = ComplianceClient::new_client(&server_host, server_port, name);
        assert_eq!(client.get_name(), name);

        let client = client.get_client().await;
        println!("{:?}", client);
        assert!(client.is_ok());
    }

    #[tokio::test]
    async fn test_client_is_ready_request() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client = ComplianceClient::new_client(&server_host, server_port, name);

        let result = client.is_ready(ReadyRequest {}).await;
        println!("{:?}", result);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().into_inner().ready, true);
    }

    #[tokio::test]
    async fn test_client_submit_flight_plan() {
        let name = "compliance";
        let (server_host, server_port) =
            lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

        let client = ComplianceClient::new_client(&server_host, server_port, name);

        let result = client
            .submit_flight_plan(FlightPlanRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            })
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

        let client = ComplianceClient::new_client(&server_host, server_port, name);

        let result = client
            .request_flight_release(FlightReleaseRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            })
            .await;

        assert!(result.is_ok());
        let result: FlightReleaseResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        assert_eq!(result.released, true);
    }
}
