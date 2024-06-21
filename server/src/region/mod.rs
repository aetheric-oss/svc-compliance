//! Provides region specific implementations for the compliance functions

#[macro_use]
pub mod macros;

cfg_if::cfg_if! {
    if #[cfg(feature = "us")] {
        pub mod us;
    } else {
        pub mod nl;
    }
}

pub mod utils;

use crate::grpc::server;
use lib_common::time::{DateTime, Utc};
use server::{FlightPlanRequest, FlightPlanResponse};
use server::{FlightReleaseRequest, FlightReleaseResponse};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis;
use tonic::{Request, Response, Status};

/// Generic region struct to be used to implement the region specific traits
#[derive(Debug, Clone)]
pub struct RegionImpl {
    /// The implemented region short code
    pub region: String,
}

/// Details of a flight restriction
#[derive(Debug, Clone)]
pub struct RestrictionDetails {
    /// The boundary vertices of the restriction
    pub vertices: Vec<gis::Coordinates>,

    /// The start time of the restriction
    pub timestamp_start: Option<DateTime<Utc>>,

    /// The end time of the restriction
    pub timestamp_end: Option<DateTime<Utc>>,

    /// The restriction type
    pub zone_type: gis::ZoneType,

    /// The maximum altitude
    pub altitude_meters_max: f32,

    /// The minimum altitude
    pub altitude_meters_min: f32,
}

/// Interface to regional authorities
#[tonic::async_trait]
pub trait RegionInterface {
    /// Return the region short code of the implementation
    fn get_region(&self) -> &str;

    /// Submit a new flight plan for the region
    fn submit_flight_plan(
        &self,
        request: FlightPlanRequest,
    ) -> Result<Response<FlightPlanResponse>, Status>;

    /// Request a flight plan release for the region
    fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status>;

    /// Refresh the in memory stored restrictions
    async fn acquire_restrictions(&self, restrictions: &mut HashMap<String, RestrictionDetails>);

    /// Refresh the in memory stored waypoints
    async fn acquire_waypoints(&self, waypoints: &mut HashMap<String, gis::Coordinates>);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_region_code() {
        lib_common::logger::get_log_handle().await;
        ut_info!("Start.");

        let region_impl = RegionImpl::default();
        cfg_if::cfg_if! {
            if #[cfg(feature = "us")] {
                assert_eq!(region_impl.region, "us");
            } else {
                assert_eq!(region_impl.region, "nl");
            }
        }

        ut_info!("Success.");
    }
}
