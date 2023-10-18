//! Region implementation for The Netherlands (NL)

use crate::grpc::server::{FlightPlanRequest, FlightPlanResponse, FlightReleaseResponse};

use super::{AuthorityResponse, FlightReleaseRequest, RequestStatus};
use crate::region::RegionInterface;
use crate::region::RestrictionDetails;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis::Coordinates;
use tonic::{Request, Response, Status};

//
// TODO(R4): Refresh intervals for receiving data from external sources
// NOTE: These may be updated to minutes or hours in the future
//
// const NL_RESTRICTION_REFRESH_INTERVAL_MS: u64 = 30000; // 30s
// const NL_WAYPOINT_REFRESH_INTERVAL_MS: u64 = 60000; // 60s

impl Default for super::RegionImpl {
    fn default() -> Self {
        Self {
            region: String::from("nl"),
        }
    }
}

/// Processes for submission to the Dutch (Netherlands) authorities
#[tonic::async_trait]
impl RegionInterface for super::RegionImpl {
    fn get_region(&self) -> &str {
        &self.region
    }

    async fn submit_flight_plan(&self, request: &flight_plan::Object) -> Result<(), Status> {
        region_info!("(submit_flight_plan)[nl] entry.");

        // TODO(R5+): Contact the authority here
        // Hardcoded for now

        Ok(())
    }

    async fn get_flight_plan_status(
        &self,
        flight_plan_id: &String,
    ) -> Result<AuthorityResponse, Status> {
        region_info!("(get_flight_plan_status)[nl] entry.");

        // TODO(R5+): Contact the authority here
        // Hardcoded for now
        Ok(AuthorityResponse {
            flight_plan_id: flight_plan_id.clone(),
            status: RequestStatus::Approved,
            timestamp: Utc::now(),
        })
    }

    /// Request flight release for a flight plan
    async fn request_flight_release(&self, request: &flight_plan::Object) -> Result<(), Status> {
        region_info!("(request_flight_release)[nl] entry.");

        // TODO(R5+): Contact the authority here
        // Hardcoded for now

        Ok(())
    }

    /// Get the status of a flight release request
    async fn get_flight_release_status(
        &self,
        flight_plan_id: &String,
    ) -> Result<AuthorityResponse, Status> {
        region_info!("(get_flight_release_status)[nl] entry.");

        // TODO(R5+): Contact the authority here
        // Hardcoded for now
        Ok(AuthorityResponse {
            flight_plan_id: flight_plan_id.clone(),
            status: RequestStatus::Approved,
            timestamp: Utc::now(),
        })
    }

    async fn acquire_restrictions(&self, restrictions: &mut HashMap<String, RestrictionDetails>) {
        //
        // TODO(R4): This is currently hardcoded. This should be replaced with a call to
        //  an API.
        //
        let mut from_remote: HashMap<String, RestrictionDetails> = HashMap::new();

        let vertices = vec![
            (4.9158, 52.3751),
            (4.9157, 52.3750),
            (4.9164, 52.3749),
            (4.9164, 52.3751),
        ];

        from_remote.insert(
            "ARROW-NL-TFR-ZONE".to_string(),
            RestrictionDetails {
                vertices: vertices
                    .into_iter()
                    .map(|(x, y)| Coordinates {
                        latitude: y,
                        longitude: x,
                    })
                    .collect(),
                timestamp_end: Some(Utc::now() + Duration::hours(1)),
                timestamp_start: Some(Utc::now()),
            },
        );

        let vertices = vec![
            (4.9159, 52.3743),
            (4.9169, 52.3749),
            (4.9165, 52.3751),
            (4.9166, 52.3755),
            (4.9191, 52.3751),
            (4.9166, 52.3730),
            (4.9143, 52.3732),
            (4.9132, 52.3749),
            (4.9145, 52.3758),
            (4.9152, 52.3757),
            (4.9149, 52.3751),
            (4.9155, 52.3748),
        ];

        from_remote.insert(
            "ARROW-NL-NOFLY-ZONE".to_string(),
            RestrictionDetails {
                vertices: vertices
                    .into_iter()
                    .map(|(longitude, latitude)| Coordinates {
                        latitude,
                        longitude,
                    })
                    .collect(),
                timestamp_end: None,
                timestamp_start: None,
            },
        );

        //
        // END HARDCODE
        //
        restrictions.retain(|k, _| from_remote.contains_key(k));
        for (label, details) in from_remote.into_iter() {
            restrictions.insert(label, details);
        }
    }

    async fn acquire_waypoints(&self, waypoints: &mut HashMap<String, Coordinates>) {
        //
        // TODO(R4): This is currently hardcoded. This should be replaced with a call to an API
        //

        // Amsterdam Drone Lab Waypoints
        let from_remote: Vec<(f32, f32)> = vec![
            // Valid waypoints
            (4.9160, 52.3745),
            (4.9156, 52.3749),
            (4.9153, 52.3752),
            (4.9156, 52.3753),
            // Waypoint within the TFR
            (4.9161, 52.3750),
        ];

        let from_remote: HashMap<String, Coordinates> = from_remote
            .iter()
            .enumerate()
            .map(|(i, (longitude, latitude))| {
                (
                    format!("ARROW-WEG-{}", i),
                    Coordinates {
                        latitude: *latitude,
                        longitude: *longitude,
                    },
                )
            })
            .collect();

        //
        // END HARDCODE
        //

        waypoints.retain(|k, _| from_remote.contains_key(k));
        for (label, details) in from_remote.into_iter() {
            waypoints.insert(label, details);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::region::RegionImpl;

    #[tokio::test]
    async fn test_region_code() {
        crate::get_log_handle().await;
        ut_info!("(test_region_code)[nl] Start.");

        let region_impl = RegionImpl::default();
        assert_eq!(region_impl.region, "nl");

        ut_info!("(test_region_code)[nl] Success.");
    }

    #[tokio::test]
    async fn test_submit_flight_plan() {
        crate::get_log_handle().await;
        ut_info!("(test_submit_flight_plan)[nl] Start.");

        let region = RegionImpl::default();
        let result = region.submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        });

        assert!(result.is_ok());
        let result: FlightPlanResponse = result.unwrap().into_inner();
        ut_debug!("(test_submit_flight_plan)[nl] Result: {:?}", result);
        assert_eq!(result.submitted, true);

        ut_info!("(test_submit_flight_plan)[nl] Success.");
    }

    #[tokio::test]
    async fn test_request_flight_release() {
        crate::get_log_handle().await;
        ut_info!("(test_request_flight_release)[nl] Start.");

        let region = RegionImpl::default();
        let result = region.request_flight_release(tonic::Request::new(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        }));

        assert!(result.is_ok());
        let result: FlightReleaseResponse = result.unwrap().into_inner();
        ut_debug!("(test_request_flight_release)[nl] Result: {:?}", result);
        assert_eq!(result.released, true);

        ut_info!("(test_request_flight_release)[nl] Success.");
    }

    #[tokio::test]
    async fn test_acquire_restrictions() {
        crate::get_log_handle().await;
        ut_info!("(test_acquire_restrictions)[nl] Start.");

        let region = RegionImpl::default();
        let mut cache = HashMap::<String, RestrictionDetails>::new();
        region.acquire_restrictions(&mut cache).await;
        ut_debug!("(test_acquire_restrictions)[nl] Cache content: {:?}", cache);
        assert!(cache.keys().len() > 0);

        ut_info!("(test_acquire_restrictions)[nl] Success.");
    }

    #[tokio::test]
    async fn test_refresh_waypoints() {
        crate::get_log_handle().await;
        ut_info!("(test_refresh_waypoints)[nl] Start.");

        let region = RegionImpl::default();
        let mut cache = HashMap::<String, Coordinates>::new();
        region.acquire_waypoints(&mut cache).await;
        assert!(cache.keys().len() > 0);

        ut_info!("(test_refresh_waypoints)[nl] Success.");
    }
}
