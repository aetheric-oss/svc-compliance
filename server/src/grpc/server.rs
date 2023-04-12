///module svc_storage generated from svc-storage.proto
pub mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}

use grpc_server::rpc_service_server::{RpcService, RpcServiceServer};
use grpc_server::{FlightPlanRequest, FlightPlanResponse};
use grpc_server::{FlightReleaseRequest, FlightReleaseResponse};
use grpc_server::{FlightRestriction, RestrictionsRequest, RestrictionsResponse};
use grpc_server::{ReadyRequest, ReadyResponse};
use grpc_server::{Waypoint, WaypointsRequest, WaypointsResponse};

use svc_compliance::shutdown_signal;

use crate::region;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

///Implementation of gRPC endpoints
pub struct ServiceImpl {
    region: Box<dyn region::RegionInterface + Send + Sync>,
    restrictions: Arc<Mutex<Vec<FlightRestriction>>>,
    waypoints: Arc<Mutex<Vec<Waypoint>>>,
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

    async fn request_waypoints(
        &self,
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        grpc_info!("(grpc request_waypoints) entry.");
        self.region
            .request_waypoints(self.waypoints.clone(), request)
    }

    async fn request_restrictions(
        &self,
        request: Request<RestrictionsRequest>,
    ) -> Result<Response<RestrictionsResponse>, Status> {
        grpc_info!("(grpc request_restrictions) entry.");
        self.region
            .request_restrictions(self.restrictions.clone(), request)
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

    let region = get_region_impl(config)?;
    let restrictions = Arc::new(Mutex::new(Vec::new()));
    let waypoints = Arc::new(Mutex::new(Vec::new()));

    // TODO(R4): Move these to a thread and allow to loop
    region.refresh_restrictions(restrictions.clone()).await;
    region.refresh_waypoints(waypoints.clone()).await;

    let imp = ServiceImpl {
        region,
        restrictions,
        waypoints,
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
