//! Provides region specific implementations for the compliance functions

#[macro_use]
pub mod macros;

cfg_if::cfg_if! {
    if #[cfg(feature = "nl")] {
        pub mod nl;
    } else {
        pub mod us;
    }
}

#[allow(dead_code)]
pub mod utils;

use crate::grpc::server;
use server::{FlightPlanRequest, FlightPlanResponse};
use server::{FlightReleaseRequest, FlightReleaseResponse};
use server::{FlightRestriction, RestrictionsRequest, RestrictionsResponse};
use server::{Waypoint, WaypointsRequest, WaypointsResponse};
use std::sync::{Arc, Mutex};
use tonic::{Request, Response, Status};

/// Generic region struct to be used to implement the region specific traits
#[derive(Debug, Clone)]
pub struct RegionImpl {
    /// The implemented region short code
    pub region: String,
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

    /// Request the region's waypoints
    fn request_waypoints(
        &self,
        waypoints: Arc<Mutex<Vec<Waypoint>>>,
        request: Request<WaypointsRequest>,
    ) -> Result<Response<WaypointsResponse>, Status> {
        region_info!("(request_waypoints) entry.");

        let mut result = match waypoints.lock() {
            Ok(guard) => guard.clone(),
            Err(_poisoned) => {
                region_error!("(request_waypoints) Could not lock waypoints.");
                // poisoned.into_inner()
                return Err(Status::internal("Could not lock waypoints."));
            }
        };

        let request = request.into_inner();
        if let Some(filter) = request.filter {
            result.retain(|x| {
                let mut keep = true;
                if let Some(min) = filter.min {
                    keep &= x.latitude >= min.latitude;
                    keep &= x.longitude >= min.longitude;
                }

                if let Some(max) = filter.max {
                    keep &= x.latitude <= max.latitude;
                    keep &= x.longitude <= max.longitude;
                }

                keep
            });
        }

        Ok(Response::new(WaypointsResponse { waypoints: result }))
    }

    /// Request the region's restrictions
    fn request_restrictions(
        &self,
        restrictions: Arc<Mutex<Vec<FlightRestriction>>>,
        request: Request<RestrictionsRequest>,
    ) -> Result<Response<RestrictionsResponse>, Status> {
        region_info!("(request_restrictions) entry.");

        let mut result = match restrictions.lock() {
            Ok(guard) => guard.clone(),
            Err(_poisoned) => {
                region_error!("(request_waypoints) Could not lock restrictions.");
                // poisoned.into_inner()
                return Err(Status::internal("Could not lock restrictions."));
            }
        };

        let request = request.into_inner();
        if let Some(filter) = request.filter {
            result.retain(|x| {
                // Keep any restriction that has at least one vertex
                // within the window provided by the request filter
                x.vertices.iter().any(|v| {
                    let mut keep = true;
                    if let Some(min) = filter.min {
                        keep &= v.latitude >= min.latitude;
                        keep &= v.longitude >= min.longitude;
                    }

                    if let Some(max) = filter.max {
                        keep &= v.latitude <= max.latitude;
                        keep &= v.longitude <= max.longitude;
                    }

                    keep
                })
            });
        }

        Ok(Response::new(RestrictionsResponse {
            restrictions: result,
        }))
    }

    /// Refresh the in memory stored restrictions
    async fn refresh_restrictions(&self, restrictions: Arc<Mutex<Vec<FlightRestriction>>>);
    /// Refresh the in memory stored waypoints
    async fn refresh_waypoints(&self, waypoints: Arc<Mutex<Vec<Waypoint>>>);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::grpc::server::{Coordinate, CoordinateFilter, FlightRestriction, Waypoint};
    use crate::region::RegionImpl;

    #[test]
    fn test_region_code() {
        let region_impl = RegionImpl::default();
        cfg_if::cfg_if! {
            if #[cfg(feature = "nl")] {
                assert_eq!(region_impl.region, "nl");
            } else {
                assert_eq!(region_impl.region, "us");
            }
        }
    }

    #[tokio::test]
    async fn ut_request_restrictions_coordinate_filter() {
        let filter_min = Coordinate {
            latitude: -19.999,
            longitude: 1.999,
        };
        let filter_max = Coordinate {
            latitude: -9.999,
            longitude: 19.999,
        };
        let filter = CoordinateFilter {
            min: Some(filter_min),
            max: Some(filter_max),
        };

        let restrictions = Arc::new(Mutex::new(vec![
            FlightRestriction {
                identifier: "1".to_string(),
                vertices: vec![
                    // Outside the filter window
                    Coordinate {
                        latitude: filter_max.latitude + 0.001,
                        longitude: filter_max.longitude + 0.001,
                    },
                    // One vertex in the filter window, this restriction should be returned
                    Coordinate {
                        latitude: filter_min.latitude,
                        longitude: filter_max.longitude,
                    },
                    // Outside the filter window
                    Coordinate {
                        latitude: filter_min.latitude - 0.001,
                        longitude: filter_min.longitude - 0.001,
                    },
                ],
                altitude_meters_min: 0,
                altitude_meters_max: 1000,
                reason: "Test".to_string(),
                restriction_type: "Test".to_string(),
                timestamp_start: None,
                timestamp_end: None,
            },
            FlightRestriction {
                identifier: "2".to_string(),
                vertices: vec![
                    // Outside the filter window
                    Coordinate {
                        latitude: filter_max.latitude + 0.001,
                        longitude: filter_max.longitude + 0.001,
                    },
                    // Outside the filter window
                    Coordinate {
                        latitude: filter_min.latitude - 0.001,
                        longitude: filter_min.longitude - 0.001,
                    },
                    // Outside the filter window
                    Coordinate {
                        latitude: filter_min.latitude - 0.001,
                        longitude: filter_max.longitude + 0.001,
                    },
                ],
                altitude_meters_min: 0,
                altitude_meters_max: 1000,
                reason: "Test".to_string(),
                restriction_type: "Test".to_string(),
                timestamp_start: None,
                timestamp_end: None,
            },
        ]));

        // Test that only one of the two restrictions is returned
        let request = Request::new(RestrictionsRequest {
            filter: Some(filter),
        });

        let region = RegionImpl::default();
        let result = region.request_restrictions(restrictions.clone(), request);
        let Ok(response) = result else {
            panic!();
        };
        let response = response.into_inner();

        assert_eq!(response.restrictions.len(), 1);
        assert_eq!(response.restrictions[0].identifier, "1");

        // Test that none of the restrictions are returned
        let request = Request::new(RestrictionsRequest {
            filter: Some(CoordinateFilter {
                min: Some(Coordinate {
                    latitude: filter_max.latitude + 0.002,
                    longitude: filter_max.longitude + 0.002,
                }),
                max: None,
            }),
        });

        let result = region.request_restrictions(restrictions.clone(), request);
        let Ok(response) = result else {
            panic!();
        };
        let response = response.into_inner();

        assert_eq!(response.restrictions.len(), 0);

        // Test that both restrictions are returned
        let request = Request::new(RestrictionsRequest {
            filter: Some(CoordinateFilter {
                min: None,
                max: None,
            }),
        });

        let result = region.request_restrictions(restrictions.clone(), request);
        let Ok(response) = result else {
            panic!();
        };
        let response = response.into_inner();
        assert_eq!(response.restrictions.len(), 2);
    }

    #[tokio::test]
    async fn ut_request_waypoints() {
        let filter_min = Coordinate {
            latitude: 9.999,
            longitude: 9.999,
        };
        let filter_max = Coordinate {
            latitude: 19.999,
            longitude: 19.999,
        };
        let filter = CoordinateFilter {
            min: Some(filter_min),
            max: Some(filter_max),
        };

        let waypoints = Arc::new(Mutex::new(vec![
            // Outside filter
            Waypoint {
                identifier: "1".to_string(),
                latitude: filter_min.latitude - 0.00001,
                longitude: filter_min.longitude - 0.00001,
            },
            // This waypoint is on the edge of the filter window
            Waypoint {
                identifier: "2".to_string(),
                latitude: filter_min.latitude,
                longitude: filter_min.longitude,
            },
            // Outside filter
            Waypoint {
                identifier: "3".to_string(),
                latitude: filter_max.longitude + 0.00001,
                longitude: filter_max.longitude,
            },
            // This waypoint is on the edge of the filter window
            Waypoint {
                identifier: "4".to_string(),
                latitude: filter_max.latitude,
                longitude: filter_max.longitude,
            },
        ]));

        let request = Request::new(WaypointsRequest {
            filter: Some(filter),
        });

        let region = RegionImpl::default();
        let result = region.request_waypoints(waypoints.clone(), request);
        let Ok(response) = result else {
            panic!();
        };
        let response = response.into_inner();

        let valid_waypoint_1 = waypoints.lock().unwrap()[1].clone();
        let valid_waypoint_2 = waypoints.lock().unwrap()[3].clone();
        assert_eq!(response.waypoints.len(), 2);
        assert_eq!(response.waypoints[0].identifier.as_str(), "2");
        assert_eq!(response.waypoints[0].latitude, valid_waypoint_1.latitude);
        assert_eq!(response.waypoints[0].longitude, valid_waypoint_1.longitude);
        assert_eq!(response.waypoints[1].identifier.as_str(), "4");
        assert_eq!(response.waypoints[1].latitude, valid_waypoint_2.latitude);
        assert_eq!(response.waypoints[1].longitude, valid_waypoint_2.longitude);
    }
}
