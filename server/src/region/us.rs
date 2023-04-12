use crate::grpc::server::grpc_server;
use grpc_server::Coordinate;
use grpc_server::FlightRestriction;
use grpc_server::Waypoint;
use grpc_server::{FlightPlanRequest, FlightPlanResponse};
use grpc_server::{FlightReleaseRequest, FlightReleaseResponse};

use crate::region::RegionInterface;
use chrono::{Duration, Utc};
use lib_common::time::datetime_to_timestamp;
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

//
// TODO(R4): Refresh intervals for receiving data from external sources
// NOTE: These may be updated to minutes or hours in the future
//
// const US_RESTRICTION_REFRESH_INTERVAL_MS: u64 = 30000; // 30s
// const US_WAYPOINT_REFRESH_INTERVAL_MS: u64 = 60000; // 60s

/// Processes for submission to the US authorities
pub struct USImpl {}

#[tonic::async_trait]
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

    async fn refresh_restrictions(&self, restrictions: Arc<Mutex<Vec<FlightRestriction>>>) {
        //
        // TODO(R4): This is all currently hardcoded. This should be replaced with a call to
        //  an API.
        // let response = hyper::request::get("https://www.caa.api.com/no-fly").unwrap();
        // etc.

        let mut tmp = vec![];

        // No Fly Zone
        {
            let vertices = vec![
                (30.93212056685634, -104.04716408989421),
                (30.931361203527537, -104.04286047256427),
                (30.931692016016083, -104.04203656211823),
                (30.933255841376866, -104.03999431601257),
                (30.931902532458363, -104.03988037095085),
                (30.932601744457997, -104.03746122964117),
                (30.93524817798048, -104.03942459070413),
                (30.934158036680692, -104.04652424454781),
            ];

            let vertices = vertices
                .iter()
                .map(|(lat, lon)| Coordinate {
                    latitude: *lat,
                    longitude: *lon,
                })
                .collect();

            tmp.push(FlightRestriction {
                identifier: "ARROW-US-NOFLY-ZONE".to_string(),
                vertices,
                altitude_meters_min: 0,
                altitude_meters_max: 6000,
                timestamp_end: None,
                timestamp_start: None,
                restriction_type: "no-fly".to_string(),
                reason: "Residential.".to_string(),
            });
        }

        // Temporary Flight Restriction
        {
            let vertices = vec![
                (30.93109003018109, -104.04248469001575),
                (30.93161918826889, -104.0399078085524),
                (30.930125481322523, -104.03908789172316),
                (30.929937929025687, -104.04051688962555),
            ];

            let vertices = vertices
                .iter()
                .map(|(lat, lon)| Coordinate {
                    latitude: *lat,
                    longitude: *lon,
                })
                .collect();

            let now = Utc::now();
            let until = now + Duration::days(1);
            let Some(now) = datetime_to_timestamp(&now) else {
                region_error!("([us] refresh_restrictions) Could not convert timestamp.");
                return;
            };

            let Some(until) = datetime_to_timestamp(&until) else {
                region_error!("([us] refresh_restrictions) Could not convert timestamp.");
                return;
            };

            tmp.push(FlightRestriction {
                identifier: "ARROW-US-TFR-ZONE".to_string(),
                vertices,
                altitude_meters_min: 0,
                altitude_meters_max: 6000,
                timestamp_end: Some(now),
                timestamp_start: Some(until),
                restriction_type: "TFR".to_string(),
                reason: "Purposeful no-fly for testing.".to_string(),
            });
        }

        let n_items = tmp.len();

        // TODO(R4): When this is on a separate thread, allow to loop
        // loop {
        {
            *restrictions.lock().unwrap() = tmp.clone();
        }

        region_info!(
            "([us] refresh_restrictions) refreshed restrictions, found {}.",
            n_items
        );

        //     std::thread::sleep(std::time::Duration::from_millis(
        //         US_RESTRICTION_REFRESH_INTERVAL_MS,
        //     ));
        // }
    }

    async fn refresh_waypoints(&self, waypoints: Arc<Mutex<Vec<Waypoint>>>) {
        //
        // TODO(R4): This is currently hardcoded. This should be replaced with a call to an API
        //

        // West Texas Hardcode
        let tmp: Vec<(f64, f64)> = vec![
            // Ideal waypoint around hardcoded flight restriction
            (30.931177107045443, -104.0428517004023),
            // waypoint within the hardcoded flight restriction
            (30.930882385083812, -104.04126652786576),
        ];

        let mut count = 0;
        let tmp: Vec<Waypoint> = tmp
            .iter()
            .map(|(lat, lon)| {
                count += 1;
                Waypoint {
                    identifier: format!("ARROW-WAY-{}", count),
                    latitude: *lat,
                    longitude: *lon,
                }
            })
            .collect();

        let n_items = tmp.len();

        // TODO(R4): When this is on a separate thread, allow to loop
        // loop {
        {
            *waypoints.lock().unwrap() = tmp;
        }

        region_info!(
            "([us] refresh_waypoints) waypoints refreshed, found: {}",
            n_items
        );

        //     std::thread::sleep(std::time::Duration::from_millis(
        //         US_WAYPOINT_REFRESH_INTERVAL_MS,
        //     ))
        // }
    }
}
