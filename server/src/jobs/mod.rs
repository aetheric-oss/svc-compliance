//! Implementation of the responsibilities of this microservice.
//! It includes:
//! - Requesting flight releases from regional authorities
//! - Requesting flight plan approvals from regional authorities
//! - Requesting up-to-date waypoints and corridors from regional authorities
//! - Requesting up-to-date no-fly zones from regional authorities

#[macro_use]
pub mod macros;
pub mod corridors;
pub mod flights;
pub mod restrictions;

use chrono::{DateTime, Utc};

/// Sleeps until the given timestamp
fn sleep_until(trigger: &DateTime<Utc>) {
    let now = Utc::now();

    if now >= *trigger {
        jobs_warn!("(flight_release_loop) Loop iteration took longer than expected interval.");

        return;
    }

    let sleep_duration = *trigger - now;
    jobs_debug!(
        "(flight_release_loop) Sleeping for {} seconds.",
        sleep_duration.num_seconds()
    );

    std::thread::sleep(sleep_duration.to_std().unwrap());
}
