//! Submits flight plans to the regional authority

//! Requests permission to fly for upcoming approved flight plans.

use crate::config::Config;
use crate::region::{RegionInterface, RequestStatus};
use chrono::{DateTime, Duration, Utc};
use std::collections::HashMap;

use svc_storage_client_grpc::prelude::flight_plan;
use svc_storage_client_grpc::prelude::{AdvancedSearchFilter, FieldMask};
use svc_storage_client_grpc::simple_service::Client;

struct FlightDetails {
    flight_plan_id: String,
    departure_time: DateTime<Utc>,
}

struct StatusLib<C, F>
where
    C: Fn(&String, &DateTime<Utc>, &svc_storage_client_grpc::Clients) -> F,
    F: std::future::Future,
{
    mark_approved: C,
    mark_denied: C,
}

impl TryFrom<flight_plan::Object> for FlightDetails {
    type Error = ();

    fn try_from(object: flight_plan::Object) -> Result<Self, ()> {
        let Some(data) = object.data else {
            jobs_error!("(TryFrom FlightDetails) Flight plan has no data.");
            return Err(());
        };

        let Some(departure_time) = data.scheduled_departure else {
            jobs_error!("(TryFrom FlightDetails) Flight plan has no departure time.");
            return Err(());
        };

        Ok(Self {
            flight_plan_id: object.id,
            departure_time: departure_time.into(),
        })
    }
}

/// Removes expired flight release requests from the cache
fn remove_expired_requests(cache: &mut HashMap<String, FlightDetails>) {
    // Don't make requests for unapproved flight plans whose arrival time has passed
    // Might allow a flight release for an active flight plan, especially one that changes midair
    let to_remove = cache
        .iter()
        .filter_map(|(id, details)| {
            if details.departure_time < Utc::now() {
                Some(id.clone())
            } else {
                None
            }
        })
        .collect::<Vec<String>>();

    for id in to_remove {
        jobs_warn!(
            "(flight_release_loop) Flight plan {} expired before release was evaluated.",
            id
        );
        cache.remove(&id);
    }
}

async fn get_new_requests(
    lookahead_limit_seconds: u32,
    cache: &mut HashMap<String, FlightDetails>,
    region: &Box<dyn RegionInterface + Send + Sync>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    // get flights needing flight release from svc-storage
    // only request flight plans departing within the next hour
    // allow flight release request until planned arrival time
    let trigger_datetime = Utc::now() + Duration::seconds(lookahead_limit_seconds.into());
    let filter = AdvancedSearchFilter::search_is_null("flight_release_approval".to_string())
        .and_less_or_equal(
            "scheduled_departure".to_string(),
            trigger_datetime.to_string(),
        )
        .and_greater_or_equal("scheduled_departure".to_string(), Utc::now().to_string())
        .and_not_in(
            "flight_status".to_owned(),
            vec![
                (flight_plan::FlightStatus::Finished as i32).to_string(),
                (flight_plan::FlightStatus::Cancelled as i32).to_string(),
            ],
        );

    let upcoming_flights = match storage_clients.flight_plan.search(filter).await {
        Err(e) => {
            jobs_error!(
                "(flight_release_loop) Failed to get flight plans from storage: {:?}",
                e
            );
            return;
        }
        Ok(response) => response.into_inner().list,
    };

    // register the unapproved flight plans with the local cache
    for object in upcoming_flights.into_iter() {
        if cache.contains_key(&object.id) {
            continue;
        }

        match region.submit_flight_plan(&object).await {
            Ok(()) => {
                jobs_info!(
                    "(flight_release_loop) Flight plan {} submitted to authority.",
                    object.id
                );
            }
            Err(e) => {
                jobs_error!(
                    "(flight_release_loop) Flight plan {} failed to submit to authority: {:?}",
                    object.id,
                    e
                );
                continue;
            }
        }

        let flight_plan_id = object.id.clone();
        if let Ok(details) = FlightDetails::try_from(object) {
            cache.insert(flight_plan_id, details);
        };
    }
}

/// Marks a flight release request as approved
async fn mark_release_approved(
    flight_plan_id: &String,
    timestamp: &DateTime<Utc>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    jobs_info!(
        "(flight_release_loop) Flight plan {} approved by authority.",
        flight_plan_id
    );

    let data = flight_plan::Data {
        flight_release_approval: Some((*timestamp).into()),
        ..Default::default()
    };

    let updated = flight_plan::UpdateObject {
        id: flight_plan_id.clone(),
        data: Some(data),
        mask: Some(FieldMask {
            paths: vec!["flight_release_approval".to_owned()],
        }),
    };

    // We know the authority approved, and we have it in logs, so not too big an issue if fails to locally record
    let response = match storage_clients.flight_plan.update(updated).await {
        Ok(response) => response.into_inner(),
        Err(e) => {
            jobs_error!(
                "(flight_release_loop) Could not update flight plan (storage client error): {}",
                flight_plan_id
            );
            return;
        }
    };

    match response.validation_result {
        Some(result) => {
            if !result.success {
                jobs_warn!(
                    "(flight_release_loop) Flight plan update validation failure for ID: {}",
                    flight_plan_id
                );
            }
        }
        None => {
            jobs_warn!(
                "(flight_release_loop) Flight plan update validation result is invalid for ID: {}",
                flight_plan_id
            );
        }
    }
}

/// Marks a flight release request as denied
async fn mark_release_denied(
    flight_plan_id: &String,
    _timestamp: &DateTime<Utc>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    jobs_info!(
        "(flight_release_loop) Flight plan {} denied.",
        flight_plan_id
    );

    // TODO(R4): Update rationale in storage record
    // TODO(R4): RequestStatus field in storage record
    // If possible, cancel the flight plan
    let data = flight_plan::Data {
        flight_status: flight_plan::FlightStatus::Cancelled as i32,
        ..Default::default()
    };

    let updated = flight_plan::UpdateObject {
        id: flight_plan_id.clone(),
        data: Some(data),
        mask: Some(FieldMask {
            paths: vec!["flight_status".to_owned()],
        }),
    };

    let response = match storage_clients.flight_plan.update(updated).await {
        Err(e) => {
            jobs_error!("(flight_release_loop) Could not update flight plan (storage client error) {flight_plan_id}: {e}");
            return;
        }
        Ok(response) => response.into_inner(),
    };

    match response.validation_result {
        Some(result) => {
            if !result.success {
                jobs_warn!(
                    "(flight_release_loop) Flight plan update validation failure for ID: {}",
                    flight_plan_id
                );
            }
        }
        None => {
            jobs_warn!(
                "(flight_release_loop) Flight plan update validation result is invalid for ID: {}",
                flight_plan_id
            );
        }
    }
}

/// Marks a flight release request as approved
async fn mark_plan_approved(
    flight_plan_id: &String,
    timestamp: &DateTime<Utc>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    jobs_info!(
        "(mark_plan_approved) Flight plan {} approved by authority.",
        flight_plan_id
    );

    let data = flight_plan::Data {
        flight_plan_approval: Some((*timestamp).into()),
        ..Default::default()
    };

    let updated = flight_plan::UpdateObject {
        id: flight_plan_id.clone(),
        data: Some(data),
        mask: Some(FieldMask {
            paths: vec!["flight_plan_approval".to_owned()],
        }),
    };

    // We know the authority approved, and we have it in logs, so not too big an issue if fails to locally record
    let response = match storage_clients.flight_plan.update(updated).await {
        Ok(response) => response.into_inner(),
        Err(e) => {
            jobs_error!(
                "(mark_plan_approved) Could not update flight plan (storage client error): {}",
                flight_plan_id
            );
            return;
        }
    };

    match response.validation_result {
        Some(result) => {
            if !result.success {
                jobs_warn!(
                    "(flight_release_loop) Flight plan update validation failure for ID: {}",
                    flight_plan_id
                );
            }
        }
        None => {
            jobs_warn!(
                "(flight_release_loop) Flight plan update validation result is invalid for ID: {}",
                flight_plan_id
            );
        }
    }
}

/// Marks a flight plan request as denied
async fn mark_plan_denied(
    flight_plan_id: &String,
    _timestamp: &DateTime<Utc>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    jobs_info!("(mark_plan_denied) Flight plan {} denied.", flight_plan_id);

    // send a request to svc-scheduler to cancel the flight plan
}

/// Checks the status of each flight release request
async fn check_status(
    lib: &StatusLib,
    cache: &mut HashMap<String, FlightDetails>,
    region: &Box<dyn RegionInterface + Send + Sync>,
    storage_clients: &svc_storage_client_grpc::Clients,
) {
    let mut to_remove = vec![];
    for flight_plan_id in cache.keys() {
        let response = match region.get_flight_plan_status(flight_plan_id).await {
            Ok(response) => response,
            Err(e) => {
                jobs_error!(
                    "(flight_release_loop) Failed to get flight release status from authority for ID {flight_plan_id}: {e}"
                );

                continue;
            }
        };

        match response.status {
            RequestStatus::Pending => {
                jobs_debug!("(check_status) Flight plan {} pending.", flight_plan_id);
                continue;
            }
            RequestStatus::Approved => {
                (lib.mark_approved)(flight_plan_id, &response.timestamp, storage_clients).await;
            }
            RequestStatus::Denied => {
                (lib.mark_denied)(flight_plan_id, &response.timestamp, storage_clients).await;
            }
        }

        to_remove.push(flight_plan_id.clone());
    }

    for flight_plan_id in to_remove {
        cache.remove(&flight_plan_id);
    }
}

/// Requests flight release permission for each flight
pub async fn common_loop(
    lib: StatusLib,
    interval_seconds: u16,
    lookahead_limit_seconds: u32,
    storage_clients: svc_storage_client_grpc::Clients,
    region: Box<dyn RegionInterface + Send + Sync>,
) {
    jobs_info!(
        "(common_loop) Starting loop with interval: {} seconds.",
        interval_seconds
    );

    let mut cache: HashMap<String, FlightDetails> = HashMap::new();
    loop {
        let next_trigger = Utc::now() + Duration::seconds(interval_seconds.into());

        remove_expired_requests(&mut cache);
        get_new_requests(
            lookahead_limit_seconds,
            &mut cache,
            &region,
            &storage_clients,
        )
        .await;
        check_status(&lib, &mut cache, &region, &storage_clients).await;

        super::sleep_until(&next_trigger);
    }
}

pub async fn flight_release_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let storage_clients =
        svc_storage_client_grpc::Clients::new(config.storage_host_grpc, config.storage_port_grpc);

    let lib = StatusLib {
        mark_approved: mark_release_approved,
        mark_denied: mark_release_denied,
    };

    common_loop(
        lib,
        config.interval_seconds_flight_releases,
        config.flight_release_lookahead_seconds,
        storage_clients,
        region,
    )
    .await;
}

pub async fn flight_plan_loop(config: Config, region: Box<dyn RegionInterface + Send + Sync>) {
    let storage_clients =
        svc_storage_client_grpc::Clients::new(config.storage_host_grpc, config.storage_port_grpc);

    let lib = StatusLib {
        mark_approved: mark_plan_approved,
        mark_denied: mark_plan_denied,
    };

    common_loop(
        lib,
        config.interval_seconds_flight_plans,
        config.flight_plan_lookahead_seconds,
        storage_clients,
        region,
    )
    .await;
}
