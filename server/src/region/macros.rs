//! log macro's for region logging

/// Writes a debug! message to the app::region logger
#[macro_export]
macro_rules! region_debug {
    ($($arg:tt)+) => {
        log::debug!(target: "app::region", $($arg)+)
    };
}

/// Writes an info! message to the app::region logger
#[macro_export]
macro_rules! region_info {
    ($($arg:tt)+) => {
        log::info!(target: "app::region", $($arg)+)
    };
}

/// Writes an warn! message to the app::region logger
#[macro_export]
macro_rules! region_warn {
    ($($arg:tt)+) => {
        log::warn!(target: "app::region", $($arg)+)
    };
}

/// Writes an error! message to the app::region logger
#[macro_export]
macro_rules! region_error {
    ($($arg:tt)+) => {
        log::error!(target: "app::region", $($arg)+)
    };
}
