//! gRPC server implementation

///  module svc_storage generated from svc-storage.proto
mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}

pub use grpc_server::rpc_service_server::{RpcService, RpcServiceServer};
pub use grpc_server::{Coordinate, CoordinateFilter};
pub use grpc_server::{FlightPlanRequest, FlightPlanResponse};
pub use grpc_server::{FlightReleaseRequest, FlightReleaseResponse};
pub use grpc_server::{FlightRestriction, RestrictionsRequest, RestrictionsResponse};
pub use grpc_server::{ReadyRequest, ReadyResponse};
pub use grpc_server::{Waypoint, WaypointsRequest, WaypointsResponse};

use crate::config::Config;
use crate::region::RegionInterface;
use crate::shutdown_signal;

use core::fmt;
use std::net::SocketAddr;
use std::sync::{Arc, Mutex};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// struct to implement the gRPC server functions
pub struct ServerImpl {
    region: Box<dyn RegionInterface + Send + Sync>,
    restrictions: Arc<Mutex<Vec<FlightRestriction>>>,
    waypoints: Arc<Mutex<Vec<Waypoint>>>,
}
impl fmt::Debug for ServerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerImpl")
            .field("region", &"RegionInterface (not printable)")
            .field(
                "restrictions",
                &format_args!("{:?}", self.restrictions.lock().unwrap()),
            )
            .field(
                "waypoints",
                &format_args!("{:?}", self.waypoints.lock().unwrap()),
            )
            .finish()
    }
}

impl Default for ServerImpl {
    fn default() -> Self {
        let region = Box::<crate::region::RegionImpl>::default();
        let restrictions = Arc::new(Mutex::new(Vec::new()));
        let waypoints = Arc::new(Mutex::new(Vec::new()));

        Self {
            region,
            restrictions,
            waypoints,
        }
    }
}

#[cfg(not(feature = "mock_server"))]
#[tonic::async_trait]
impl RpcService for ServerImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_warn!(
            "([{}] is_ready) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(is_ready) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        grpc_warn!(
            "([{}] submit_flight_plan) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(submit_flight_plan) request: {:?}", request);
        self.region.submit_flight_plan(request)
    }

    async fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        grpc_warn!(
            "([{}] request_flight_release) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(request_flight_release) request: {:?}", request);
        self.region.request_flight_release(request)
    }

    async fn request_waypoints(
        &self,
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        grpc_warn!(
            "([{}] request_waypoints) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(request_waypoints) request: {:?}", request);
        self.region
            .request_waypoints(self.waypoints.clone(), request)
    }

    async fn request_restrictions(
        &self,
        request: Request<RestrictionsRequest>,
    ) -> Result<Response<RestrictionsResponse>, Status> {
        grpc_warn!(
            "([{}] request_restrictions) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(request_restrictions) request: {:?}", request);
        self.region
            .request_restrictions(self.restrictions.clone(), request)
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
pub async fn grpc_server(config: Config) {
    grpc_debug!("(grpc_server) entry.");

    // Grpc Server
    let grpc_port = config.docker_port_grpc;
    let full_grpc_addr: SocketAddr = match format!("[::]:{}", grpc_port).parse() {
        Ok(addr) => addr,
        Err(e) => {
            grpc_error!("(grpc_server) Failed to parse gRPC address: {}", e);
            return;
        }
    };

    let imp = ServerImpl::default();
    // TODO(R4): Move these to a thread and allow to loop
    imp.region
        .refresh_restrictions(imp.restrictions.clone())
        .await;
    imp.region.refresh_waypoints(imp.waypoints.clone()).await;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    health_reporter
        .set_serving::<RpcServiceServer<ServerImpl>>()
        .await;

    //start server
    grpc_info!(
        "([{}] grpc_server) Starting gRPC services on: {}.",
        imp.region.get_region(),
        full_grpc_addr
    );
    match Server::builder()
        .add_service(health_service)
        .add_service(RpcServiceServer::new(imp))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc"))
        .await
    {
        Ok(_) => grpc_info!("(grpc_server) gRPC server running at: {}.", full_grpc_addr),
        Err(e) => {
            grpc_error!("(grpc_server) Could not start gRPC server: {}", e);
        }
    };
}

#[cfg(feature = "mock_server")]
#[tonic::async_trait]
impl RpcService for ServerImpl {
    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
    ) -> Result<Response<ReadyResponse>, Status> {
        grpc_warn!(
            "([{}] is_ready MOCK) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(is_ready MOCK) request: {:?}", request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        grpc_warn!(
            "([{}] submit_flight_plan MOCK) compliance server.",
            self.region.get_region()
        );
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
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        grpc_warn!(
            "([{}] request_flight_release MOCK) compliance server.",
            self.region.get_region()
        );
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
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        grpc_warn!(
            "([{}] request_waypoints MOCK) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(request_waypoints MOCK) request: {:?}", request);
        Ok(tonic::Response::new(WaypointsResponse {
            waypoints: vec![],
        }))
    }

    async fn request_restrictions(
        &self,
        request: Request<RestrictionsRequest>,
    ) -> Result<Response<RestrictionsResponse>, Status> {
        grpc_warn!(
            "([{}] request_restrictions MOCK) compliance server.",
            self.region.get_region()
        );
        grpc_debug!("(request_restrictions MOCK) request: {:?}", request);
        Ok(tonic::Response::new(RestrictionsResponse {
            restrictions: vec![],
        }))
    }
}

#[cfg(test)]
mod tests {
    use super::grpc_server::*;
    use super::*;

    #[test]
    fn test_region_code() {
        let imp = ServerImpl::default();
        cfg_if::cfg_if! {
            if #[cfg(feature = "nl")] {
                assert_eq!(imp.region.get_region(), "nl");
            } else {
                assert_eq!(imp.region.get_region(), "us");
            }
        }
    }

    #[tokio::test]
    async fn test_grpc_server_is_ready() {
        let imp = ServerImpl::default();
        let result = imp.is_ready(Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);
    }

    #[tokio::test]
    async fn test_grpc_submit_flight_plan() {
        let imp = ServerImpl::default();
        let result = imp
            .submit_flight_plan(Request::new(FlightPlanRequest {
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
        let imp = ServerImpl::default();
        let result = imp
            .request_flight_release(Request::new(FlightReleaseRequest {
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
        let imp = ServerImpl::default();

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
        let result = imp
            .request_waypoints(Request::new(WaypointsRequest {
                filter: Some(filter),
            }))
            .await;

        assert!(result.is_ok());
        let result: WaypointsResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        // Would not have loaded any waypoints in this case
        assert_eq!(result.waypoints.len(), 0);
    }

    #[tokio::test]
    async fn test_grpc_request_restrictions() {
        let imp = ServerImpl::default();

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
        let result = imp
            .request_restrictions(tonic::Request::new(RestrictionsRequest {
                filter: Some(filter),
            }))
            .await;

        assert!(result.is_ok());
        let result: RestrictionsResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        // Would not have loaded any restrictions in this case
        assert_eq!(result.restrictions.len(), 0);
    }
}
