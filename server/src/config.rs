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

    /// interval in seconds to refresh no-fly zones
    pub interval_seconds_refresh_zones: u16,

    /// interval in seconds to refresh waypoints
    pub interval_seconds_refresh_waypoints: u16,

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
            gis_port_grpc: 50051,
            interval_seconds_refresh_zones: 30,
            interval_seconds_refresh_waypoints: 30,
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
        let default_config = Config::default();

        config::Config::builder()
            .set_default("docker_port_grpc", default_config.docker_port_grpc)?
            .set_default("log_config", default_config.log_config)?
            .set_default(
                "interval_seconds_refresh_zones",
                default_config.interval_seconds_refresh_zones,
            )?
            .set_default(
                "interval_seconds_refresh_waypoints",
                default_config.interval_seconds_refresh_waypoints,
            )?
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
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_config_from_default) Start.");

        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.gis_host_grpc, String::from("svc-gis"));
        assert_eq!(config.gis_port_grpc, 50051);
        assert_eq!(config.interval_seconds_refresh_zones, 30);
        assert_eq!(config.interval_seconds_refresh_waypoints, 30);
        assert_eq!(config.log_config, String::from("log4rs.yaml"));
        assert!(config.amqp.url.is_none());
        assert!(config.amqp.pool.is_none());

        ut_info!("(test_config_from_default) Success.");
    }

    #[tokio::test]
    async fn test_config_from_env() {
        lib_common::logger::get_log_handle().await;
        ut_info!("(test_config_from_env) Start.");

        std::env::set_var("DOCKER_PORT_GRPC", "6789");
        std::env::set_var("GIS_HOST_GRPC", "svc-gis");
        std::env::set_var("GIS_PORT_GRPC", "6798");
        std::env::set_var("INTERVAL_SECONDS_REFRESH_ZONES", "40");
        std::env::set_var("INTERVAL_SECONDS_REFRESH_WAYPOINTS", "40");
        std::env::set_var("LOG_CONFIG", "config_file.yaml");
        std::env::set_var("AMQP__URL", "amqp://test_rabbitmq:5672");
        std::env::set_var("AMQP__POOL__MAX_SIZE", "32");

        let config = Config::try_from_env();
        assert!(config.is_ok());
        let config = config.unwrap();

        assert_eq!(config.docker_port_grpc, 6789);
        assert_eq!(config.gis_host_grpc, String::from("svc-gis"));
        assert_eq!(config.gis_port_grpc, 6798);
        assert_eq!(config.interval_seconds_refresh_zones, 40);
        assert_eq!(config.interval_seconds_refresh_waypoints, 40);
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
