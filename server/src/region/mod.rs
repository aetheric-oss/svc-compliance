#[macro_use]
pub mod macros;
pub mod nl;
pub mod us;

#[allow(dead_code)]
pub mod utils;

use crate::grpc::server::grpc_server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
    WaypointsRequest, WaypointsResponse,
};
use tonic::{Request, Response, Status};

/// Interface to regional authorities
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
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status>;
}
