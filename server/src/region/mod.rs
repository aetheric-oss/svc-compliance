//! Provides region specific implementations for the compliance functions

use svc_storage_client_grpc::prelude::flight_plan;

#[macro_use]
pub mod macros;

cfg_if::cfg_if! {
    if #[cfg(feature = "nl")] {
        pub mod nl;
    } else {
        pub mod us;
    }
}

pub mod utils;

use chrono::{DateTime, Utc};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis;
use tonic::Status;

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
}

/// Status of a request to a regional authority
#[derive(Debug, Clone, Copy)]
pub enum RequestStatus {
    /// The request was approved
    Approved,

    /// The request was denied
    Denied,

    /// The request is pending
    Pending,
}

/// Response from a regional authority
#[derive(Debug, Clone)]
pub struct AuthorityResponse {
    /// The flight plan ID
    pub flight_plan_id: String,

    /// The status of the request
    pub status: RequestStatus,

    /// The timestamp of the response
    pub timestamp: DateTime<Utc>,
}

/// Interface to regional authorities
#[tonic::async_trait]
pub trait RegionInterface {
    /// Return the region short code of the implementation
    fn get_region(&self) -> &str;

    /// Submit a new flight plan for the region
    async fn submit_flight_plan(&self, request: &flight_plan::Object) -> Result<(), Status>;

    /// Request the status of a flight plan
    async fn get_flight_plan_status(
        &self,
        flight_plan_id: &String,
    ) -> Result<AuthorityResponse, Status>;

    /// Request a flight plan release for the region
    async fn request_flight_release(&self, request: &flight_plan::Object) -> Result<(), Status>;

    /// Request the status of a flight plan release for the region
    async fn get_flight_release_status(
        &self,
        flight_plan_id: &String,
    ) -> Result<AuthorityResponse, Status>;

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
        crate::get_log_handle().await;
        ut_info!("(test_region_code) Start.");

        let region_impl = RegionImpl::default();
        cfg_if::cfg_if! {
            if #[cfg(feature = "nl")] {
                assert_eq!(region_impl.region, "nl");
            } else {
                assert_eq!(region_impl.region, "us");
            }
        }

        ut_info!("(test_region_code) Success.");
    }
}
