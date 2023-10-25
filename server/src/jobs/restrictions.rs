//! Requests no-fly zones from the regional authority

use crate::config::Config;
use crate::region::{RegionInterface, RestrictionDetails};
use std::collections::HashMap;
use svc_gis_client_grpc::prelude::*;

/// Results of updating restrictions
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum UpdateRestrictionsStatus {
    /// Restrictions were updated
    Success,

    /// No restrictions were updated
    NoRestrictions,

    /// Request to gRPC server failed
    RequestFailure,
}

async fn update_restrictions(
    host: String,
    port: u16,
    restrictions: &HashMap<String, RestrictionDetails>,
) -> UpdateRestrictionsStatus {
    let mut zones: Vec<gis::NoFlyZone> = vec![];

    for (label, details) in restrictions.iter() {
        let time_start = details.timestamp_start.map(|t| t.into());
        let time_end = details.timestamp_end.map(|t| t.into());

        zones.push(gis::NoFlyZone {
            label: label.clone(),
            vertices: details
                .vertices
                .iter()
                .map(|v| gis::Coordinates {
                    latitude: v.latitude,
                    longitude: v.longitude,
                })
                .collect(),
            time_start,
            time_end,
        });
    }

    if zones.is_empty() {
        jobs_warn!("(update_restrictions) No restrictions to update.");
        return UpdateRestrictionsStatus::NoRestrictions;
    }

    let client = GisClient::new_client(&host, port, "gis");
    match client
        .update_no_fly_zones(gis::UpdateNoFlyZonesRequest { zones })
        .await
    {
        Ok(response) => {
            jobs_debug!("(update_restrictions) Got response: {:?}", response);
            UpdateRestrictionsStatus::Success
        }
        Err(e) => {
            jobs_error!("(update_restrictions) {:?}", e);
            UpdateRestrictionsStatus::RequestFailure
        }
    }
}

/// Periodically pulls down restrictions from the regional interface and
///  pushes them to the GIS microservice
pub async fn restrictions_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let host = config.gis_host_grpc;
    let port = config.gis_port_grpc;
    let mut cache: HashMap<String, RestrictionDetails> = HashMap::new();

    jobs_info!(
        "(restrictions_loop) Starting loop with interval: {} seconds.",
        config.interval_seconds_refresh_zones
    );

    loop {
        region.acquire_restrictions(&mut cache).await;
        update_restrictions(host.clone(), port, &cache).await;
        std::thread::sleep(std::time::Duration::from_secs(
            config.interval_seconds_refresh_zones as u64,
        ));
    }
}
