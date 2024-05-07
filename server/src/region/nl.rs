//! Region implementation for The Netherlands (NL)

use crate::grpc::server::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};

use crate::region::RegionInterface;
use crate::region::RestrictionDetails;
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis::{Coordinates, ZoneType};
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
        region_info!("(submit_flight_plan)[nl] entry.");

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
        region_info!("(request_flight_release)[nl] entry.");

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
        let zones: Vec<(&str, Vec<(f64, f64)>)> = vec![
            (
                "schiphol",
                vec![
                    (4.7091866, 52.3827247),
                    (4.6507947, 52.3294647),
                    (4.7560834, 52.2572307),
                    (4.8234058, 52.3214912),
                    (4.7091866, 52.3827247),
                ],
            ),
            (
                "hoorn",
                vec![
                    (5.0232724, 52.6317085),
                    (5.1069102, 52.6347298),
                    (5.1036471, 52.6459798),
                    (5.1227104, 52.6501458),
                    (5.0948883, 52.6829387),
                    (5.0306572, 52.6710736),
                    (5.0358094, 52.6534782),
                    (5.0102200, 52.6393135),
                    (5.0229289, 52.6317085),
                    (5.0232724, 52.6317085),
                ],
            ),
            (
                "enkuizen",
                vec![
                    (5.2790303, 52.6962570),
                    (5.2867586, 52.6928238),
                    (5.3121763, 52.7067632),
                    (5.2864152, 52.7253768),
                    (5.2604823, 52.7204902),
                    (5.2611692, 52.7089473),
                    (5.2790303, 52.6962570),
                ],
            ),
            (
                "paleis",
                vec![
                    (4.8822724, 52.3688393),
                    (4.8832170, 52.3781666),
                    (4.9007345, 52.3777998),
                    (4.9001335, 52.3680532),
                    (4.8822724, 52.3688393),
                ],
            ),
            (
                "lelystad",
                vec![
                    (5.4339886, 52.5337670),
                    (5.4030752, 52.4942783),
                    (5.5226069, 52.4806893),
                    (5.5325679, 52.5329316),
                    (5.4339886, 52.5337670),
                ],
            ),
            (
                "almere",
                vec![
                    (5.1146438, 52.3818865),
                    (5.1273526, 52.3447783),
                    (5.1541442, 52.3240093),
                    (5.2294815, 52.3269469),
                    (5.3468376, 52.4038851),
                    (5.2664628, 52.4350834),
                    (5.2279928, 52.4262914),
                    (5.1551747, 52.3672147),
                    (5.1359397, 52.3869157),
                    (5.1146438, 52.3818865),
                ],
            ),
            (
                "volendam",
                vec![
                    (5.0379751, 52.4928151),
                    (5.0702624, 52.4842438),
                    (5.0912148, 52.5071317),
                    (5.0742124, 52.5225926),
                    (5.0465621, 52.5207125),
                    (5.0379751, 52.4928151),
                ],
            ),
            (
                "monnickendam",
                vec![
                    (5.0302965, 52.4649000),
                    (5.0241138, 52.4491049),
                    (5.0486728, 52.4423563),
                    (5.0503902, 52.4547018),
                    (5.0423184, 52.4629128),
                    (5.0302965, 52.4649000),
                ],
            ),
            (
                "marken",
                vec![
                    (5.0982680, 52.4568985),
                    (5.1042360, 52.4545449),
                    (5.1099464, 52.4564540),
                    (5.1135959, 52.4619715),
                    (5.1092594, 52.4657889),
                    (5.1006295, 52.4612655),
                    (5.0982680, 52.4568985),
                ],
            ),
        ];

        let from_remote = zones
            .into_iter()
            .map(|(zone_name, zone_vertices)| {
                let data = RestrictionDetails {
                    vertices: zone_vertices
                        .into_iter()
                        .map(|(x, y)| Coordinates {
                            latitude: y,
                            longitude: x,
                        })
                        .collect(),
                    timestamp_start: None,
                    timestamp_end: None,
                    altitude_meters_min: 0.0,
                    altitude_meters_max: 1000.0,
                    zone_type: ZoneType::Restriction,
                };

                (format!("ARROW-NL-NOFLY-{}", zone_name), data)
            })
            .collect::<HashMap<String, RestrictionDetails>>();

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
        let from_remote: Vec<(f64, f64)> = vec![
            // Valid waypoints
            (4.9160, 52.3745),
            (4.9156, 52.3749),
            (4.9153, 52.3752),
            (4.9156, 52.3753),
            // Waypoint within the TFR
            (4.9161, 52.3750),
            // Others
            (5.2021015, 52.9635294),
            (5.2570586, 52.8691297),
            (5.3724685, 52.7878146),
            (5.4164342, 52.6780473),
            (5.3559814, 52.6047156),
            (5.2735457, 52.5513062),
            (5.2021015, 52.4944873),
            (5.1361530, 52.4174978),
            (5.0784480, 52.3638604),
            (5.0234941, 52.2858024),
            (4.9699542, 52.2542885),
            (4.9095214, 52.2742499),
            (4.9075041, 52.3103678),
            (5.2844417, 52.6672220),
            (5.2133705, 52.6122211),
            (5.1309348, 52.5930378),
            (5.0732299, 52.5796880),
            (5.0057699, 52.5550636),
            (4.9314804, 52.5341847),
            (4.8697511, 52.5015937),
            (4.8566987, 52.4677236),
            (4.8148570, 52.5329316),
            (4.7921165, 52.5742635),
            (4.8375440, 52.6038816),
            (4.8918141, 52.6330629),
            (4.9550148, 52.6701368),
            (5.0051632, 52.6959449),
            (5.0553115, 52.7092593),
            (5.1189528, 52.7421118),
            (5.1892695, 52.8060822),
            (5.5227080, 52.7703701),
            (5.5213340, 52.7080113),
            (5.5309515, 52.6380634),
            (5.5007251, 52.5888665),
            (5.3784456, 52.5270836),
            (5.3399756, 52.4727431),
            (5.3866891, 52.4258727),
            (5.4457680, 52.3630218),
            (5.6862054, 52.4091213),
            (5.7865021, 52.4568462),
            (5.8538245, 52.5496361),
            (5.0791343, 53.0230443),
            (4.9870812, 53.0519455),
            (4.9280023, 53.1030944),
            (4.9664723, 53.1739423),
            (5.0502818, 53.2380987),
            (5.2357621, 53.3103651),
            (5.3621634, 53.3513711),
            (5.5023041, 53.3857856),
            (5.6026008, 53.4005262),
            (5.7606025, 53.4136247),
            (4.8892276, 52.3661141),
            (4.8931777, 52.3793717),
            (4.8784938, 52.3742892),
            (4.9035680, 52.3718788),
            (4.7266651, 52.2700483),
            (4.8111616, 52.3481340),
            (4.6383902, 52.3676339),
            (4.8558143, 52.2631146),
            (4.8805450, 52.3166644),
            (4.7946745, 52.2286406),
            (4.6164074, 52.3109975),
            (4.7022778, 52.4068174),
            (5.0968408, 52.6132634),
            (5.1521414, 52.6491043),
            (5.1342804, 52.6751331),
            (5.0985582, 52.6969853),
            (5.0456620, 52.6903267),
            (5.0034138, 52.6645152),
            (4.9955137, 52.6330629),
            (5.0463490, 52.6134719),
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
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_region_code)[nl] Start.");

        let region_impl = RegionImpl::default();
        assert_eq!(region_impl.region, "nl");

        ut_info!("(test_region_code)[nl] Success.");
    }

    #[tokio::test]
    async fn test_submit_flight_plan() {
        lib_common::logger::get_log_handle().await;
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
        lib_common::logger::get_log_handle().await;
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
        lib_common::logger::get_log_handle().await;
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
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_refresh_waypoints)[nl] Start.");

        let region = RegionImpl::default();
        let mut cache = HashMap::<String, Coordinates>::new();
        region.acquire_waypoints(&mut cache).await;
        assert!(cache.keys().len() > 0);

        ut_info!("(test_refresh_waypoints)[nl] Success.");
    }
}
