use crate::grpc::server::grpc_server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse, Waypoint,
    WaypointsRequest, WaypointsResponse,
};
use crate::region::RegionInterface;

use tonic::{Request, Response, Status};

/// Processes for submission to the US authorities
pub struct USImpl {}
impl RegionInterface for USImpl {
    fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        region_info!("([us] submit_flight_plan) entry.");
        // TODO R3 implement
        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightPlanResponse {
            flight_plan_id,
            submitted: true,
            result: None,
        }))
    }

    fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        region_info!("([us] request_flight_release) entry.");
        // TODO R3 implement
        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightReleaseResponse {
            flight_plan_id,
            released: true,
            result: None,
        }))
    }

    fn request_waypoints(
        &self,
        _request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        region_info!("([nl] request_waypoints) entry.");

        // TODO(R4): This is hardcoded in R3. Eventually an external API should be called.
        let waypoints: Vec<Waypoint> = vec![];

        Ok(Response::new(WaypointsResponse { waypoints }))
    }
}
