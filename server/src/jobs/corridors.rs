//! Requests up-to-date corridors and waypoints from the regional authority

use crate::config::Config;
use crate::region::RegionInterface;
use chrono::{Duration, Utc};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::gis;
use svc_gis_client_grpc::prelude::*;

/// Results of updating waypoints
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateWaypointsStatus {
    /// Waypoints were updated
    Success,

    /// No waypoints were updated
    NoWaypoints,

    /// Request to gRPC server failed
    RequestFailure,
}

async fn gis_update_waypoints(
    host: String,
    port: u16,
    waypoints: &HashMap<String, gis::Coordinates>,
) -> UpdateWaypointsStatus {
    let nodes: Vec<gis::Waypoint> = waypoints
        .iter()
        .map(|(label, coordinates)| gis::Waypoint {
            label: label.clone(),
            location: Some(*coordinates),
        })
        .collect();

    if nodes.is_empty() {
        jobs_warn!("(update_waypoints) No waypoints to update.");
        return UpdateWaypointsStatus::NoWaypoints;
    }

    let client = GisClient::new_client(&host, port, "gis");
    match client
        .update_waypoints(gis::UpdateWaypointsRequest { waypoints: nodes })
        .await
    {
        Ok(response) => {
            jobs_debug!("(update_waypoints) Got response: {:?}", response);
            UpdateWaypointsStatus::Success
        }
        Err(e) => {
            jobs_error!("(update_waypoints) {:?}", e);
            UpdateWaypointsStatus::RequestFailure
        }
    }
}

/// Periodically pulls down waypoints from the regional interface and
///  pushes them to the GIS microservice
pub async fn waypoints_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let host = config.gis_host_grpc;
    let port = config.gis_port_grpc;

    jobs_info!(
        "(waypoints_loop) Starting loop with interval: {} seconds.",
        config.interval_seconds_refresh_waypoints
    );

    let mut cache: HashMap<String, gis::Coordinates> = HashMap::new();

    loop {
        let next_trigger =
            Utc::now() + Duration::seconds(config.interval_seconds_refresh_waypoints.into());

        region.acquire_waypoints(&mut cache).await;
        gis_update_waypoints(host.clone(), port, &cache).await;
        super::sleep_until(&next_trigger);
    }
}
