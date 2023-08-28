//! gRPC server implementation

///  module svc_storage generated from svc-storage.proto
mod grpc_server {
    #![allow(unused_qualifications, missing_docs)]
    tonic::include_proto!("grpc");
}

pub use crate::amqp::init_mq;
use crate::region::RestrictionDetails;
pub use grpc_server::rpc_service_server::{RpcService, RpcServiceServer};
pub use grpc_server::{FlightPlanRequest, FlightPlanResponse};
pub use grpc_server::{FlightReleaseRequest, FlightReleaseResponse};
pub use grpc_server::{ReadyRequest, ReadyResponse};
use svc_gis_client_grpc::prelude::*;

use crate::config::Config;
use crate::region::RegionInterface;
use crate::shutdown_signal;

use core::fmt;
use std::collections::HashMap;
use std::net::SocketAddr;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

/// struct to implement the gRPC server functions
#[allow(missing_debug_implementations)]
pub struct ServerImpl {
    /// AMQP channel
    pub mq_channel: Option<lapin::Channel>,

    /// Region interface
    pub region: Box<dyn RegionInterface + Send + Sync>,
}

/// Results of updating restrictions
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateRestrictionsStatus {
    /// Restrictions were updated
    Success,

    /// No restrictions were updated
    NoRestrictions,

    /// Request to gRPC server failed
    RequestFailure,
}

/// Results of updating waypoints
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateWaypointsStatus {
    /// Waypoints were updated
    Success,

    /// No waypoints were updated
    NoWaypoints,

    /// Request to gRPC server failed
    RequestFailure,
}

impl fmt::Debug for ServerImpl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ServerImpl")
            .field("region", &"RegionInterface (not printable)")
            .finish()
    }
}

#[cfg(not(feature = "stub_server"))]
#[tonic::async_trait]
impl RpcService for ServerImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        request: Request<ReadyRequest>,
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
        let request = request.into_inner();
        let response = self.region.submit_flight_plan(request.clone())?;

        // send flight plan to AMQP
        if let Some(mq_channel) = &self.mq_channel {
            let Ok(payload) = serde_json::to_vec(&request) else {
                grpc_error!("(submit_flight_plan) could not serialize flight plan.");
                return Ok(response);
            };

            let result = mq_channel
                .basic_publish(
                    crate::amqp::EXCHANGE_NAME_FLIGHTPLAN,
                    crate::amqp::QUEUE_NAME_CARGO,
                    lapin::options::BasicPublishOptions::default(),
                    &payload,
                    lapin::BasicProperties::default(),
                )
                .await;

            match result {
                Ok(_) => grpc_info!("(submit_flight_plan) telemetry pushed to RabbitMQ."),
                Err(e) => {
                    grpc_error!("(submit_flight_plan) telemetry push to RabbitMQ failed: {e}.")
                }
            }
        }

        Ok(response)
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
}

async fn update_waypoints(
    host: String,
    port: u16,
    waypoints: &HashMap<String, gis::Coordinates>,
) -> UpdateWaypointsStatus {
    let nodes: Vec<gis::Waypoint> = waypoints
        .iter()
        .map(|(label, coordinates)| gis::Waypoint {
            label: label.clone(),
            location: Some(*coordinates),
        })
        .collect();

    if nodes.is_empty() {
        grpc_warn!("(update_waypoints) no waypoints to update.");
        return UpdateWaypointsStatus::NoWaypoints;
    }

    let client = GisClient::new_client(&host, port, "gis");
    match client
        .update_waypoints(gis::UpdateWaypointsRequest { waypoints: nodes })
        .await
    {
        Ok(response) => {
            grpc_debug!("(update_waypoints) RESPONSE={:?}", response);
            UpdateWaypointsStatus::Success
        }
        Err(e) => {
            grpc_error!("(update_waypoints) ERROR={:?}", e);
            UpdateWaypointsStatus::RequestFailure
        }
    }
}

/// Periodically pulls down waypoints from the regional interface and
///  pushes them to the GIS microservice
pub async fn waypoints_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let host = config.gis_host_grpc;
    let port = config.gis_port_grpc;

    grpc_info!(
        "(waypoints_loop) Starting loop with interval: {} seconds.",
        config.interval_seconds_refresh_waypoints
    );

    let mut cache: HashMap<String, gis::Coordinates> = HashMap::new();

    loop {
        // Pull down waypoints from regional interface
        region.acquire_waypoints(&mut cache).await;
        update_waypoints(host.clone(), port, &cache).await;
        std::thread::sleep(std::time::Duration::from_secs(
            config.interval_seconds_refresh_waypoints as u64,
        ));
    }
}

async fn update_restrictions(
    host: String,
    port: u16,
    restrictions: &HashMap<String, RestrictionDetails>,
) -> UpdateRestrictionsStatus {
    let mut zones: Vec<gis::NoFlyZone> = vec![];

    for (label, details) in restrictions.iter() {
        let time_start = details.timestamp_start.map(|t| t.into());
        let time_end = details.timestamp_end.map(|t| t.into());

        zones.push(gis::NoFlyZone {
            label: label.clone(),
            vertices: details
                .vertices
                .iter()
                .map(|v| gis::Coordinates {
                    latitude: v.latitude,
                    longitude: v.longitude,
                })
                .collect(),
            time_start,
            time_end,
        });
    }

    if zones.is_empty() {
        grpc_warn!("(update_restrictions) no restrictions to update.");
        return UpdateRestrictionsStatus::NoRestrictions;
    }

    let client = GisClient::new_client(&host, port, "gis");
    match client
        .update_no_fly_zones(gis::UpdateNoFlyZonesRequest { zones })
        .await
    {
        Ok(response) => {
            grpc_debug!("(update_restrictions) RESPONSE={:?}", response);
            UpdateRestrictionsStatus::Success
        }
        Err(e) => {
            grpc_error!("(update_restrictions) ERROR={:?}", e);
            UpdateRestrictionsStatus::RequestFailure
        }
    }
}

/// Periodically pulls down restrictions from the regional interface and
///  pushes them to the GIS microservice
pub async fn restrictions_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let host = config.gis_host_grpc;
    let port = config.gis_port_grpc;
    let mut cache: HashMap<String, RestrictionDetails> = HashMap::new();

    grpc_info!(
        "(restrictions_loop) Starting loop with interval: {} seconds.",
        config.interval_seconds_refresh_zones
    );

    loop {
        region.acquire_restrictions(&mut cache).await;
        update_restrictions(host.clone(), port, &cache).await;
        std::thread::sleep(std::time::Duration::from_secs(
            config.interval_seconds_refresh_zones as u64,
        ));
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

    // RabbitMQ Channel
    let Ok(mq_channel) = init_mq(config.clone()).await else {
        grpc_error!("(grpc_server) could not create channel to amqp server.");
        return;
    };

    let imp = ServerImpl {
        mq_channel: Some(mq_channel),
        region: Box::<crate::region::RegionImpl>::default(),
    };

    tokio::spawn(restrictions_loop(
        config.clone(),
        Box::<crate::region::RegionImpl>::default(),
    ));

    tokio::spawn(waypoints_loop(
        config.clone(),
        Box::<crate::region::RegionImpl>::default(),
    ));

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

#[cfg(feature = "stub_server")]
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
}

#[cfg(test)]
mod tests {
    use super::grpc_server::*;
    use super::*;

    fn get_server_impl() -> ServerImpl {
        let region = Box::<crate::region::RegionImpl>::default();
        ServerImpl {
            mq_channel: None,
            region,
        }
    }

    #[tokio::test]
    async fn test_region_code() {
        let imp = get_server_impl();
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
        let imp = get_server_impl();
        let result = imp.is_ready(Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);
    }

    #[tokio::test]
    async fn test_grpc_submit_flight_plan() {
        let imp = get_server_impl();
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
        let imp = get_server_impl();
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
    async fn test_update_restrictions() {
        let host = "localhost".to_string();
        let port = 50008;

        let mut cache: HashMap<String, RestrictionDetails> = HashMap::new();
        let result = update_restrictions(host.clone(), port, &cache).await;
        assert_eq!(result, UpdateRestrictionsStatus::NoRestrictions);

        cache.insert(
            "test".to_string(),
            RestrictionDetails {
                vertices: vec![],
                timestamp_start: Some(chrono::Utc::now()),
                timestamp_end: None,
            },
        );

        let result = update_restrictions(host.clone(), port, &cache).await;
        assert_eq!(result, UpdateRestrictionsStatus::Success);
    }

    #[tokio::test]
    async fn test_update_waypoints() {
        let host = "localhost".to_string();
        let port = 50008;

        let mut cache: HashMap<String, gis::Coordinates> = HashMap::new();
        let result = update_waypoints(host.clone(), port, &cache).await;
        assert_eq!(result, UpdateWaypointsStatus::NoWaypoints);

        cache.insert(
            "ARROW-WAY-1".to_string(),
            gis::Coordinates {
                latitude: 0.0,
                longitude: 0.0,
            },
        );

        let result = update_waypoints(host.clone(), port, &cache).await;
        assert_eq!(result, UpdateWaypointsStatus::Success);
    }
}
