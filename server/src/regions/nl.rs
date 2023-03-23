use crate::region_interface::RegionInterface;
use crate::svc_compliance::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};
use tonic::{Request, Response, Status};

/// Processes for submission to the Dutch (Netherlands) authorities
pub struct NLImpl {}
impl RegionInterface for NLImpl {
    fn submit_flight_plan(
        &self,
        _request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        todo!()
    }

    fn request_flight_release(
        &self,
        _request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        todo!()
    }
}
