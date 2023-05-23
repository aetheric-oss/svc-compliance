//! # Config
//!
//! Define and implement config options for module

use anyhow::Result;
use config::{ConfigError, Environment};
use dotenv::dotenv;
use serde::Deserialize;

/// struct holding configuration options
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    /// port to be used for gRPC server
    pub docker_port_grpc: u16,
    /// path to log configuration YAML file
    pub log_config: String,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl Config {
    /// Default values for Config
    pub fn new() -> Self {
        log::warn!("Creating Config object with default values.");
        Config {
            docker_port_grpc: 50051,
            log_config: String::from("./log4rs.yaml"),
        }
    }

    /// Create a new `Config` object using environment variables
    pub fn try_from_env() -> Result<Self, ConfigError> {
        // read .env file if present
        dotenv().ok();

        config::Config::builder()
            .set_default("docker_port_grpc", 50051)?
            .set_default("log_config", String::from("./log4rs.yaml"))?
            .add_source(Environment::default().separator("__"))
            .build()?
            .try_deserialize()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_default() {
        let config = Config::default();

        assert_eq!(config.docker_port_grpc, 50051);
        assert_eq!(config.log_config, String::from("./log4rs.yaml"));
    }

    #[tokio::test]
    async fn test_config_from_env() {
        async move {
            std::env::set_var("DOCKER_PORT_GRPC", "6789");
            std::env::set_var("LOG_CONFIG", "config_file.yaml");

            let config = Config::try_from_env();
            assert!(config.is_ok());
            let config = config.unwrap();

            assert_eq!(config.docker_port_grpc, 6789);
            assert_eq!(config.log_config, String::from("config_file.yaml"));
        }
        .await
    }
}
