#[macro_use]
pub mod macros;
pub mod nl;
pub mod us;

#[allow(dead_code)]
pub mod utils;

use crate::grpc::server::grpc_server;
use grpc_server::{FlightPlanRequest, FlightPlanResponse};
use grpc_server::{FlightReleaseRequest, FlightReleaseResponse};
use grpc_server::{FlightRestriction, RestrictionsRequest, RestrictionsResponse};
use grpc_server::{Waypoint, WaypointsRequest, WaypointsResponse};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

/// Interface to regional authorities
#[tonic::async_trait]
pub trait RegionInterface {
    fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status>;

    fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status>;

    fn request_waypoints(
        &self,
        waypoints: Arc<Mutex<Vec<Waypoint>>>,
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        region_info!("(request_waypoints) entry.");

        let mut result = match waypoints.lock() {
            Ok(guard) => guard.clone(),
            Err(_poisoned) => {
                region_error!("(request_waypoints) Could not lock waypoints.");
                // poisoned.into_inner()
                return Err(Status::internal("Could not lock waypoints."));
            }
        };

        let request = request.into_inner();
        if let Some(filter) = request.filter {
            result.retain(|x| {
                let mut keep = true;
                if let Some(min) = filter.min {
                    keep &= x.latitude >= min.latitude;
                    keep &= x.longitude >= min.longitude;
                }

                if let Some(max) = filter.max {
                    keep &= x.latitude <= max.latitude;
                    keep &= x.longitude <= max.longitude;
                }

                keep
            });
        }

        Ok(Response::new(WaypointsResponse { waypoints: result }))
    }

    fn request_restrictions(
        &self,
        restrictions: Arc<Mutex<Vec<FlightRestriction>>>,
        request: Request<RestrictionsRequest>,
    ) -> Result<Response<RestrictionsResponse>, Status> {
        region_info!("(request_restrictions) entry.");

        let mut result = match restrictions.lock() {
            Ok(guard) => guard.clone(),
            Err(_poisoned) => {
                region_error!("(request_waypoints) Could not lock restrictions.");
                // poisoned.into_inner()
                return Err(Status::internal("Could not lock restrictions."));
            }
        };

        let request = request.into_inner();
        if let Some(filter) = request.filter {
            result.retain(|x| {
                // Keep any restriction that has at least one vertex
                // within the window provided by the request filter
                x.vertices.iter().any(|v| {
                    let mut keep = true;
                    if let Some(min) = filter.min {
                        keep &= v.latitude >= min.latitude;
                        keep &= v.longitude >= min.longitude;
                    }

                    if let Some(max) = filter.max {
                        keep &= v.latitude <= max.latitude;
                        keep &= v.longitude <= max.longitude;
                    }

                    keep
                })
            });
        }

        Ok(Response::new(RestrictionsResponse {
            restrictions: result,
        }))
    }

    async fn refresh_restrictions(&self, restrictions: Arc<Mutex<Vec<FlightRestriction>>>);
    async fn refresh_waypoints(&self, waypoints: Arc<Mutex<Vec<Waypoint>>>);
}
