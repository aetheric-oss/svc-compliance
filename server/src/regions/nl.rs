use crate::region_interface::RegionInterface;
use crate::svc_compliance::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};
use log::info;
use tonic::{Request, Response, Status};

/// Processes for submission to the Dutch (Netherlands) authorities
pub struct NLImpl {}
impl RegionInterface for NLImpl {
    fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        info!("(NLImpl submit_flight_plan) entry.");
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
        info!("(NLImpl request_flight_release) entry.");
        // TODO R3 implement
        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightReleaseResponse {
            flight_plan_id,
            released: true,
            result: None,
        }))
    }
}
