//! Region implementation for the United States (US)

use crate::grpc::server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};

use crate::region::RegionInterface;
use crate::region::RestrictionDetails;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis::{Coordinates, ZoneType};
use tonic::{Request, Response, Status};

impl Default for super::RegionImpl {
    fn default() -> Self {
        Self {
            region: String::from("us"),
        }
    }
}

/// Processes for submission to the US authorities
#[tonic::async_trait]
impl RegionInterface for super::RegionImpl {
    fn get_region(&self) -> &str {
        &self.region
    }

    fn submit_flight_plan(
        &self,
        request: FlightPlanRequest,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        region_info!("(submit_flight_plan)[us] entry.");
        // TODO(R4) implement
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
        region_info!("(request_flight_release)[us] entry.");
        // TODO(R4) implement
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
            (30.9310, -104.0424),
            (30.9316, -104.0399),
            (30.9301, -104.039),
            (30.9299, -104.0405),
            (30.9310, -104.0424),
        ];

        let Some(delta) = Duration::try_hours(1) else {
            region_error!("Failed to create duration");
            return;
        };

        from_remote.insert(
            "ARROW-USA-TFR-ZONE".to_string(),
            RestrictionDetails {
                vertices: vertices
                    .into_iter()
                    .map(|(latitude, longitude)| Coordinates {
                        latitude,
                        longitude,
                    })
                    .collect(),
                timestamp_end: Some(Utc::now() + delta),
                timestamp_start: Some(Utc::now()),
                altitude_meters_min: 0.0,
                altitude_meters_max: 2000.0,
                zone_type: ZoneType::Restriction,
            },
        );

        let vertices = vec![
            (30.9321, -104.0471),
            (30.9313, -104.0428),
            (30.9316, -104.042),
            (30.9332, -104.0399),
            (30.9319, -104.0398),
            (30.9326, -104.0374),
            (30.9352, -104.0394),
            (30.9341, -104.0465),
            (30.9321, -104.0471),
        ];

        from_remote.insert(
            "ARROW-USA-NOFLY-ZONE".to_string(),
            RestrictionDetails {
                vertices: vertices
                    .into_iter()
                    .map(|(latitude, longitude)| Coordinates {
                        latitude,
                        longitude,
                    })
                    .collect(),
                // altitude_meters_min: 0,
                // altitude_meters_max: 6000,
                timestamp_end: None,
                timestamp_start: None,
                altitude_meters_min: 0.0,
                altitude_meters_max: 200.0,
                zone_type: ZoneType::Restriction,
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

        // West TX
        let from_remote: Vec<(f64, f64)> = vec![
            // Ideal waypoint around hardcoded flight restriction
            (30.9311, -104.0428),
            // waypoint within the hardcoded flight restriction
            (30.9308, -104.0412),
        ];

        let from_remote: HashMap<String, Coordinates> = from_remote
            .into_iter()
            .enumerate()
            .map(|(i, (longitude, latitude))| {
                (
                    format!("ARROW-WEG-{}", i),
                    Coordinates {
                        latitude,
                        longitude,
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
        ut_info!("(test_region_code)[us] Start.");

        let region_impl = RegionImpl::default();
        assert_eq!(region_impl.region, "us");

        ut_info!("(test_region_code)[us] Success.");
    }

    #[tokio::test]
    async fn test_submit_flight_plan() {
        crate::get_log_handle().await;
        ut_info!("(test_submit_flight_plan)[us] Start.");

        let region = RegionImpl::default();
        let result = region.submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        });

        assert!(result.is_ok());
        let result: FlightPlanResponse = result.unwrap().into_inner();
        ut_debug!("(test_submit_flight_plan)[us] Result: {:?}", result);
        assert_eq!(result.submitted, true);

        ut_info!("(test_submit_flight_plan)[us] Success.");
    }

    #[tokio::test]
    async fn test_request_flight_release() {
        crate::get_log_handle().await;
        ut_info!("(test_request_flight_release)[us] Start.");

        let region = RegionImpl::default();
        let result = region.request_flight_release(tonic::Request::new(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        }));

        assert!(result.is_ok());
        let result: FlightReleaseResponse = result.unwrap().into_inner();
        ut_debug!("(test_request_flight_release)[us] Result: {:?}", result);
        assert_eq!(result.released, true);

        ut_info!("(test_request_flight_release)[us] Success.");
    }

    #[tokio::test]
    async fn test_acquire_restrictions() {
        crate::get_log_handle().await;
        ut_info!("(test_acquire_restrictions)[us] Start.");

        let region = RegionImpl::default();
        let mut cache = HashMap::<String, RestrictionDetails>::new();
        region.acquire_restrictions(&mut cache).await;
        ut_debug!("(test_acquire_restrictions)[us] Cache content: {:?}", cache);
        assert!(cache.keys().len() > 0);

        ut_info!("(test_acquire_restrictions)[us] Success.");
    }

    #[tokio::test]
    async fn test_refresh_waypoints() {
        crate::get_log_handle().await;
        ut_info!("(test_refresh_waypoints)[us] Start.");

        let region = RegionImpl::default();
        let mut cache = HashMap::<String, Coordinates>::new();
        region.acquire_waypoints(&mut cache).await;
        assert!(cache.keys().len() > 0);

        ut_info!("(test_refresh_waypoints)[us] Success.");
    }
}
