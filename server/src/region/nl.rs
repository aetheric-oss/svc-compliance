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
// const NL_RESTRICTION_REFRESH_INTERVAL_MS: u64 = 30000; // 30s
// const NL_WAYPOINT_REFRESH_INTERVAL_MS: u64 = 60000; // 60s

/// Processes for submission to the Dutch (Netherlands) authorities
pub struct NLImpl {}

#[tonic::async_trait]
impl RegionInterface for NLImpl {
    fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        region_info!("([nl] submit_flight_plan) entry.");
        // TODO(R4) implement
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

    async fn refresh_restrictions(&self, restrictions: Arc<Mutex<Vec<FlightRestriction>>>) {
        //
        // TODO(R4): This is currently hardcoded. This should be replaced with a call to
        //  an API.
        //

        let mut tmp = vec![];

        //
        // Amsterdam Drone Lab No-Fly/TFR Zones
        //

        //
        // Temporary Flight Restriction
        //
        {
            let tfr_zone: Vec<(f64, f64)> = vec![
                (4.9158481, 52.3751734),
                (4.9157998, 52.3750752),
                (4.9164569, 52.3749409),
                (4.9164999, 52.3751047),
            ];

            let vertices = tfr_zone
                .iter()
                .map(|(lon, lat)| Coordinate {
                    latitude: *lat,
                    longitude: *lon,
                })
                .collect();

            let now = Utc::now();
            let until = now + Duration::hours(5);
            let Some(now) = datetime_to_timestamp(&now) else {
                region_error!("([nl] refresh_restrictions) Could not convert timestamp.");
                return;
            };

            let Some(until) = datetime_to_timestamp(&until) else {
                region_error!("([nl] refresh_restrictions) Could not convert timestamp.");
                return;
            };

            tmp.push(FlightRestriction {
                identifier: "ARROW-NL-TFR-ZONE".to_string(),
                vertices,
                altitude_meters_min: 0,
                altitude_meters_max: 2000,
                timestamp_end: Some(now),
                timestamp_start: Some(until),
                restriction_type: "TFR".to_string(),
                reason: "Test TFR.".to_string(),
            });
        }

        //
        // No-Fly
        //
        {
            let no_fly_zone: Vec<(f64, f64)> = vec![
                (4.9159741, 52.3743089),
                (4.9169827, 52.3749147),
                (4.9165696, 52.3751309),
                (4.9166715, 52.3755009),
                (4.9191499, 52.3751309),
                (4.9166822, 52.3730774),
                (4.9143541, 52.3732215),
                (4.9132517, 52.3749769),
                (4.9145097, 52.3758464),
                (4.9152178, 52.3757465),
                (4.9149576, 52.3751456),
                (4.9155074, 52.3748934),
            ];

            let vertices = no_fly_zone
                .iter()
                .map(|(lon, lat)| Coordinate {
                    latitude: *lat,
                    longitude: *lon,
                })
                .collect();

            // No-Fly Example
            tmp.push(FlightRestriction {
                identifier: "ARROW-NL-NOFLY-ZONE".to_string(),
                vertices,
                altitude_meters_min: 0,
                altitude_meters_max: 6000,
                timestamp_end: None,
                timestamp_start: None,
                restriction_type: "no-fly".to_string(),
                reason: "Test no-fly zone.".to_string(),
            });
        }

        let n_items = tmp.len();

        // TODO(R4): When this is on a separate thread, allow to loop
        // loop {
        {
            *restrictions.lock().unwrap() = tmp.clone();
        }

        region_info!(
            "([nl] refresh_restrictions) refreshed restrictions, found {}.",
            n_items
        );

        //     std::thread::sleep(std::time::Duration::from_millis(
        //         NL_RESTRICTION_REFRESH_INTERVAL_MS,
        //     ));
        // }
    }

    async fn refresh_waypoints(&self, waypoints: Arc<Mutex<Vec<Waypoint>>>) {
        //
        // TODO(R4): This is currently hardcoded. This should be replaced with a call to an API
        //

        // Amsterdam Drone Lab Waypoints
        let tmp: Vec<(f64, f64)> = vec![
            // Valid waypoints
            (4.9160036, 52.3745905),
            (4.9156925, 52.3749819),
            (4.9153733, 52.3752144),
            (4.9156845, 52.3753012),
            // Waypoint within the TFR
            (4.9161538, 52.3750703),
        ];

        let mut count = 0;
        let tmp: Vec<Waypoint> = tmp
            .iter()
            .map(|(lon, lat)| {
                count += 1;
                Waypoint {
                    identifier: format!("ARROW-WEG-{}", count),
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
            "([nl] refresh_waypoints) waypoints refreshed, found: {}",
            n_items
        );

        //     std::thread::sleep(std::time::Duration::from_millis(
        //         NL_WAYPOINT_REFRESH_INTERVAL_MS,
        //     ))
        // }
    }
}
