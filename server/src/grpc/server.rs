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
pub struct ServerImpl {
    /// AMQP channel
    pub mq_channel: Option<lapin::Channel>,

    /// Region interface
    pub region: Box<dyn RegionInterface + Send + Sync>,
}

/// Results of updating restrictions
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateRestrictionsError {
    /// Restrictions were updated
    Success,

    /// No restrictions were updated
    NoRestrictions,

    /// Request to gRPC server failed
    RequestFailure,
}

/// Results of updating waypoints
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateWaypointsError {
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
        let region = self.region.get_region();
        grpc_info!("(is_ready)[{}] compliance server.", region);
        grpc_debug!("(is_ready)[{}] [{:?}].", region, request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        let region = self.region.get_region();
        grpc_info!("(submit_flight_plan)[{}] compliance server.", region);
        grpc_debug!("(submit_flight_plan)[{}] [{:?}].", region, request);
        let request = request.into_inner();
        let response = self.region.submit_flight_plan(request.clone())?;

        // send flight plan to AMQP
        if let Some(mq_channel) = &self.mq_channel {
            let Ok(payload) = serde_json::to_vec(&request) else {
                grpc_error!("(submit_flight_plan) Could not serialize flight plan.");
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
                Ok(_) => grpc_info!("(submit_flight_plan) Telemetry pushed to RabbitMQ."),
                Err(e) => {
                    grpc_error!("(submit_flight_plan) Telemetry push to RabbitMQ failed: {e}")
                }
            }
        }

        Ok(response)
    }

    async fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        let region = self.region.get_region();
        grpc_info!("(request_flight_release)[{}] compliance server.", region);
        grpc_debug!("(request_flight_release)[{}] [{:?}].", region, request);
        self.region.request_flight_release(request)
    }
}

async fn update_waypoints(
    host: String,
    port: u16,
    waypoints: &HashMap<String, gis::Coordinates>,
) -> Result<(), UpdateWaypointsError> {
    let nodes: Vec<gis::Waypoint> = waypoints
        .iter()
        .map(|(label, coordinates)| gis::Waypoint {
            identifier: label.clone(),
            location: Some(*coordinates),
        })
        .collect();

    if nodes.is_empty() {
        grpc_warn!("(update_waypoints) No waypoints to update.");
        return Err(UpdateWaypointsError::NoWaypoints);
    }

    let response = GisClient::new_client(&host, port, "gis")
        .update_waypoints(gis::UpdateWaypointsRequest { waypoints: nodes })
        .await
        .map_err(|e| {
            grpc_error!("(update_waypoints) {:?}", e);
            UpdateWaypointsError::RequestFailure
        })?;

    grpc_info!("(update_waypoints) {:?}", response);
    Ok(())
}

/// Periodically pulls down waypoints from the regional interface and
///  pushes them to the GIS microservice
#[cfg(not(tarpaulin_include))]
pub async fn waypoints_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let host = config.gis_host_grpc;
    let port = config.gis_port_grpc;

    grpc_debug!(
        "(waypoints_loop) Starting loop with interval: {} seconds.",
        config.interval_seconds_refresh_waypoints
    );

    let mut cache: HashMap<String, gis::Coordinates> = HashMap::new();

    loop {
        // Pull down waypoints from regional interface
        region.acquire_waypoints(&mut cache).await;
        let _ = update_waypoints(host.clone(), port, &cache).await;
        std::thread::sleep(std::time::Duration::from_secs(
            config.interval_seconds_refresh_waypoints as u64,
        ));
    }
}

async fn update_restrictions(
    host: String,
    port: u16,
    restrictions: &HashMap<String, RestrictionDetails>,
) -> Result<(), UpdateRestrictionsError> {
    let mut zones: Vec<gis::Zone> = vec![];

    for (label, details) in restrictions.iter() {
        let time_start = details.timestamp_start.map(|t| t.into());
        let time_end = details.timestamp_end.map(|t| t.into());

        zones.push(gis::Zone {
            identifier: label.clone(),
            zone_type: details.zone_type as i32,
            altitude_meters_max: details.altitude_meters_max,
            altitude_meters_min: details.altitude_meters_min,
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
        grpc_warn!("(update_restrictions) No restrictions to update.");
        return Err(UpdateRestrictionsError::NoRestrictions);
    }

    let response = GisClient::new_client(&host, port, "gis")
        .update_zones(gis::UpdateZonesRequest { zones })
        .await
        .map_err(|e| {
            grpc_error!("(update_restrictions) {:?}", e);
            UpdateRestrictionsError::RequestFailure
        })?;

    grpc_info!("(update_restrictions) {:?}", response);
    Ok(())
}

/// Periodically pulls down restrictions from the regional interface and
///  pushes them to the GIS microservice
#[cfg(not(tarpaulin_include))]
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
        let _ = update_restrictions(host.clone(), port, &cache).await;
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
///     tokio::spawn(grpc_server(config, None)).await
/// }
/// ```
#[cfg(not(tarpaulin_include))]
// no_coverage: Can not be tested in unittest, should be part of integration
// tests
pub async fn grpc_server(config: Config, shutdown_rx: Option<tokio::sync::oneshot::Receiver<()>>) {
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
        grpc_error!("(grpc_server) Could not create channel to amqp server.");
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
        "(grpc_server)[{}] Starting gRPC services on: {}",
        imp.region.get_region(),
        full_grpc_addr
    );
    match Server::builder()
        .add_service(health_service)
        .add_service(RpcServiceServer::new(imp))
        .serve_with_shutdown(full_grpc_addr, shutdown_signal("grpc", shutdown_rx))
        .await
    {
        Ok(_) => grpc_info!("(grpc_server) gRPC server running at: {}", full_grpc_addr),
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
        let region = self.region.get_region();
        grpc_warn!("(is_ready MOCK)[{}] compliance server.", region);
        grpc_debug!("(is_ready MOCK)[{}] [{:?}].", region, request);
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        let region = self.region.get_region();
        grpc_warn!("(submit_flight_plan MOCK)[{}] compliance server.", region);
        grpc_debug!("(submit_flight_plan MOCK)[{}] [{:?}].", region, request);
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
        let region = self.region.get_region();
        grpc_warn!(
            "(request_flight_release MOCK)[{}] compliance server.",
            region
        );
        grpc_debug!("(request_flight_release MOCK)[{}] [{:?}].", region, request);
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
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_region_code) Start.");

        let imp = get_server_impl();
        cfg_if::cfg_if! {
            if #[cfg(feature = "us")] {
                assert_eq!(imp.region.get_region(), "us");
            } else {
                assert_eq!(imp.region.get_region(), "nl");
            }
        }

        ut_info!("(test_region_code) Success.");
    }

    #[tokio::test]
    async fn test_grpc_server_is_ready() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_grpc_server_is_ready) Start.");

        let imp = get_server_impl();
        let result = imp.is_ready(Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

        ut_info!("(test_grpc_server_is_ready) Success.");
    }

    #[tokio::test]
    async fn test_grpc_submit_flight_plan() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_grpc_submit_flight_plan) Start.");

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

        ut_info!("(test_grpc_submit_flight_plan) Success.");
    }

    #[tokio::test]
    async fn test_grpc_request_flight_release() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_grpc_request_flight_release) Start.");

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

        ut_info!("(test_grpc_request_flight_release) Success.");
    }

    #[tokio::test]
    async fn test_update_restrictions() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_update_restrictions) Start.");

        let host = "localhost".to_string();
        let port = 50008;

        let mut cache: HashMap<String, RestrictionDetails> = HashMap::new();
        let error = update_restrictions(host.clone(), port, &cache)
            .await
            .unwrap_err();
        assert_eq!(error, UpdateRestrictionsError::NoRestrictions);

        cache.insert(
            "test".to_string(),
            RestrictionDetails {
                vertices: vec![],
                timestamp_start: Some(chrono::Utc::now()),
                timestamp_end: None,
                altitude_meters_max: 0.,
                altitude_meters_min: 200.,
                zone_type: gis::ZoneType::Restriction,
            },
        );

        let _ = update_restrictions(host.clone(), port, &cache)
            .await
            .unwrap();
        ut_info!("(test_update_restrictions) Success.");
    }

    #[tokio::test]
    async fn test_update_waypoints() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_update_waypoints) Start.");

        let host = "localhost".to_string();
        let port = 50008;

        let mut cache: HashMap<String, gis::Coordinates> = HashMap::new();
        let error = update_waypoints(host.clone(), port, &cache)
            .await
            .unwrap_err();
        assert_eq!(error, UpdateWaypointsError::NoWaypoints);

        cache.insert(
            "ARROW-WAY-1".to_string(),
            gis::Coordinates {
                latitude: 0.0,
                longitude: 0.0,
            },
        );

        let _ = update_waypoints(host.clone(), port, &cache).await.unwrap();
        ut_info!("(test_update_waypoints) Success.");
    }
}
