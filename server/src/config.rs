//! # Config
//!
//! Define and implement config options for module

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use lapin::ConnectionProperties;
use serde::Deserialize;

/// struct holding configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// port to be used for gRPC server
    pub docker_port_grpc: u16,

    /// svc-gis hostname
    pub gis_host_grpc: String,

    /// svc-gis port
    pub gis_port_grpc: u16,

    /// svc-storage hostname
    pub storage_host_grpc: String,

    /// svc-storage port
    pub storage_port_grpc: u16,

    /// interval in seconds to refresh no-fly zones
    pub interval_seconds_refresh_zones: u16,

    /// interval in seconds to refresh waypoints
    pub interval_seconds_refresh_waypoints: u16,

    /// interval in seconds to request flight releases
    pub interval_seconds_flight_releases: u16,

    /// interval in seconds to submit flight plans
    pub interval_seconds_flight_plans: u16,

    /// how far in advance to request flight releases
    pub flight_release_lookahead_seconds: u32,

    /// how far in advance to submit flight plans
    pub flight_plan_lookahead_seconds: u32,

    /// path to log configuration YAML file
    pub log_config: String,

    /// AMQP Settings
    pub amqp: deadpool_lapin::Config,
}

impl Default for Config {
    fn default() -> Self {
        log::warn!("(default) Creating Config object with default values.");
        Self::new()
    }
}

impl Config {
    /// Create new configuration object with default values
    pub fn new() -> Self {
        Config {
            docker_port_grpc: 50051,
            gis_host_grpc: String::from("svc-gis"),
            gis_port_grpc: 50008,
            storage_host_grpc: String::from("svc-storage"),
            storage_port_grpc: 50009,
            interval_seconds_refresh_zones: 30,
            interval_seconds_refresh_waypoints: 31,
            interval_seconds_flight_releases: 32,
            interval_seconds_flight_plans: 33,
            flight_release_lookahead_seconds: 3600,
            flight_plan_lookahead_seconds: 7200,
            log_config: String::from("log4rs.yaml"),
            amqp: deadpool_lapin::Config {
                url: None,
                pool: None,
                connection_properties: ConnectionProperties::default(),
            },
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();

        config::Config::builder()
            .set_default("docker_port_grpc", 50051)?
            .set_default("log_config", String::from("log4rs.yaml"))?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::Config;

    #[tokio::test]
    async fn test_config_from_default() {
        crate::get_log_handle().await;
        ut_info!("(test_config_from_default) Start.");

        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.gis_host_grpc, String::from("svc-gis"));
        assert_eq!(config.gis_port_grpc, 50008);
        assert_eq!(config.storage_host_grpc, String::from("svc-storage"));
        assert_eq!(config.storage_port_grpc, 50009);
        assert_eq!(config.interval_seconds_refresh_zones, 30);
        assert_eq!(config.interval_seconds_refresh_waypoints, 31);
        assert_eq!(config.interval_seconds_flight_releases, 32);
        assert_eq!(config.interval_seconds_flight_plans, 33);
        assert_eq!(config.flight_release_lookahead_seconds, 3600);
        assert_eq!(config.flight_plan_lookahead_seconds, 7200);
        assert_eq!(config.log_config, String::from("log4rs.yaml"));
        assert!(config.amqp.url.is_none());
        assert!(config.amqp.pool.is_none());

        ut_info!("(test_config_from_default) Success.");
    }

    #[tokio::test]
    async fn test_config_from_env() {
        crate::get_log_handle().await;
        ut_info!("(test_config_from_env) Start.");

        std::env::set_var("DOCKER_PORT_GRPC", "6789");
        std::env::set_var("GIS_HOST_GRPC", "svc-gis");
        std::env::set_var("GIS_PORT_GRPC", "6798");
        std::env::set_var("STORAGE_HOST_GRPC", "svc-storage");
        std::env::set_var("STORAGE_PORT_GRPC", "6799");
        std::env::set_var("INTERVAL_SECONDS_REFRESH_ZONES", "40");
        std::env::set_var("INTERVAL_SECONDS_REFRESH_WAYPOINTS", "41");
        std::env::set_var("INTERVAL_SECONDS_FLIGHT_RELEASES", "42");
        std::env::set_var("INTERVAL_SECONDS_FLIGHT_PLANS", "43");
        std::env::set_var("FLIGHT_RELEASE_LOOKAHEAD_SECONDS", "3600");
        std::env::set_var("FLIGHT_PLAN_LOOKAHEAD_SECONDS", "7200");
        std::env::set_var("LOG_CONFIG", "config_file.yaml");
        std::env::set_var("AMQP__URL", "amqp://test_rabbitmq:5672");
        std::env::set_var("AMQP__POOL__MAX_SIZE", "32");

        let config = Config::try_from_env();
        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.docker_port_grpc, 6789);
        assert_eq!(config.gis_host_grpc, String::from("svc-gis"));
        assert_eq!(config.gis_port_grpc, 6798);
        assert_eq!(config.storage_host_grpc, String::from("svc-storage"));
        assert_eq!(config.storage_port_grpc, 6799);
        assert_eq!(config.interval_seconds_refresh_zones, 40);
        assert_eq!(config.interval_seconds_refresh_waypoints, 41);
        assert_eq!(config.interval_seconds_flight_releases, 42);
        assert_eq!(config.interval_seconds_flight_plans, 43);
        assert_eq!(config.flight_release_lookahead_seconds, 3600);
        assert_eq!(config.flight_plan_lookahead_seconds, 7200);
        assert_eq!(config.log_config, String::from("config_file.yaml"));
        assert_eq!(
            config.amqp.url,
            Some(String::from("amqp://test_rabbitmq:5672"))
        );
        assert_eq!(config.amqp.get_pool_config().max_size, 32);
        assert!(config.amqp.pool.is_some());

        ut_info!("(test_config_from_env) Success.");
    }
}
