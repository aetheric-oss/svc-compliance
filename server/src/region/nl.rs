//! Region implementation for The Netherlands (NL)

use crate::grpc::server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};

use crate::region::RegionInterface;
use crate::region::RestrictionDetails;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use svc_gis_client_grpc::Coordinates;
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

    fn submit_flight_plan(
        &self,
        request: FlightPlanRequest,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        region_info!("([nl] submit_flight_plan) entry.");

        //
        // TODO(R4) implement
        //

        let flight_plan_id = request.flight_plan_id;
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
        region_info!("([nl] request_flight_release) entry.");

        //
        // TODO(R4) implement
        //

        let flight_plan_id = request.into_inner().flight_plan_id;
        Ok(Response::new(FlightReleaseResponse {
            flight_plan_id,
            released: true,
            result: None,
        }))
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

    #[test]
    fn test_region_code() {
        let region_impl = RegionImpl::default();
        assert_eq!(region_impl.region, "nl");
    }

    #[test]
    fn test_submit_flight_plan() {
        let region = RegionImpl::default();
        let result = region.submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        });

        assert!(result.is_ok());
        let result: FlightPlanResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        assert_eq!(result.submitted, true);
    }

    #[test]
    fn test_request_flight_release() {
        let region = RegionImpl::default();
        let result = region.request_flight_release(tonic::Request::new(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        }));

        assert!(result.is_ok());
        let result: FlightReleaseResponse = result.unwrap().into_inner();
        println!("{:?}", result);
        assert_eq!(result.released, true);
    }

    #[tokio::test]
    async fn test_acquire_restrictions() {
        let region = RegionImpl::default();
        let mut cache = HashMap::<String, RestrictionDetails>::new();
        region.acquire_restrictions(&mut cache).await;
        assert!(cache.keys().len() > 0);
    }

    #[tokio::test]
    async fn test_refresh_waypoints() {
        let region = RegionImpl::default();
        let mut cache = HashMap::<String, Coordinates>::new();
        region.acquire_waypoints(&mut cache).await;
        assert!(cache.keys().len() > 0);
    }
}
