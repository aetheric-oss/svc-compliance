///module svc_storage generated from svc-storage.proto
pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}

use grpc_server::{
    rpc_service_server::{RpcService, RpcServiceServer},
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
    ReadyRequest, ReadyResponse,
};

use svc_compliance::shutdown_signal;

use crate::region;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
pub struct ServiceImpl {
    region: Box<dyn region::RegionInterface + Send + Sync>,
}

#[tonic::async_trait]
impl RpcService for ServiceImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_info!("(grpc is_ready) entry.");
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        grpc_info!("(grpc submit_flight_plan) entry.");
        self.region.submit_flight_plan(request)
    }

    async fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        grpc_info!("(grpc request_flight_release) entry.");
        self.region.request_flight_release(request)
    }
}

///Returns region implementation based on REGION_CODE environment variable
fn get_region_impl(
    config: crate::config::Config,
) -> Result<Box<dyn region::RegionInterface + Send + Sync>, ()> {
    grpc_info!("(get_region_impl) entry.");
    match config.region_code.as_str() {
        "us" => Ok(Box::new(region::us::USImpl {})),
        "nl" => Ok(Box::new(region::nl::NLImpl {})),
        _ => {
            grpc_error!("(get_region) Unknown region: {}.", config.region_code);
            Err(())
        }
    }
}

/// Starts the grpc servers for this microservice using the provided configuration
///
/// # Example:
/// ```
/// use svc_compliance::grpc::server::grpc_server;
/// use svc_compliance::config::Config;
/// async fn example() -> Result<(), tokio::task::JoinError> {
///     let config = Config::default();
///     tokio::spawn(grpc_server(config)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
pub async fn server(config: crate::config::Config) -> Result<(), ()> {
    grpc_info!("(grpc_server) entry.");

    // GRPC Server
    let grpc_port = config.docker_port_grpc;

    let addr = format!("[::]:{}", grpc_port);
    let Ok(full_grpc_addr) = addr.parse() else {
        grpc_error!("(grpc_server) invalid address: {:?}, exiting.", addr);
        return Err(());
    };

    let imp = ServiceImpl {
        region: get_region_impl(config)?,
    };

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<RpcServiceServer<ServiceImpl>>()
        .await;

    //start server
    grpc_info!("(grpc) hosted at {}.", full_grpc_addr);
    let _ = Server::builder()
        .add_service(health_service)
        .add_service(RpcServiceServer::new(imp))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc"))
        .await;

    Ok(())
}
