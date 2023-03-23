use crate::svc_compliance::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
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
}
