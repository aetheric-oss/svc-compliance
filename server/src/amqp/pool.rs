//! AMQP connection pool implementation

use super::AMQPError;
use deadpool_lapin::{Object, Pool, Runtime};

/// Represents a pool of connections to a amqp server
///
/// The [`AMQPPool`] struct provides a managed pool of connections to a amqp/rabbitmq server.
/// It allows clients to acquire and release connections from the pool and handles
/// connection management, such as connection pooling and reusing connections.
#[derive(Clone, Debug)]
pub struct AMQPPool {
    /// The underlying pool of AMQP connections.
    pool: Pool,
}

impl AMQPPool {
    /// Create a new AMQP pool
    pub fn new(config: crate::config::Config) -> Result<Self, AMQPError> {
        let cfg: deadpool_lapin::Config = config.amqp.clone();
        let details = cfg.url.clone().ok_or_else(|| {
            amqp_error!("No connection address found.");
            amqp_debug!("Available config: {:?}", &config.amqp);
            AMQPError::MissingConfiguration
        })?;

        amqp_info!("Creating pool at [{:?}]...", details);
        let pool = cfg.create_pool(Some(Runtime::Tokio1)).map_err(|e| {
            amqp_error!("Could not create pool: {}", e);
            AMQPError::CouldNotConnect
        })?;

        Ok(AMQPPool { pool })
    }

    /// Get a connection from the pool
    #[cfg(not(tarpaulin_include))]
    // no_coverage: (Rnever) not unit testable, only integration tests
    pub async fn get_connection(&self) -> Result<Object, AMQPError> {
        self.pool.get().await.map_err(|e| {
            amqp_error!("Could not connect to deadpool: {}", e);
            AMQPError::CouldNotConnect
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_amqp_pool_new() {
        lib_common::logger::get_log_handle().await;
        ut_info!("Start.");

        let mut config = crate::config::Config::default();
        let error = AMQPPool::new(config.clone()).unwrap_err();
        assert_eq!(error, AMQPError::MissingConfiguration);

        config.amqp.url = Some("amqp://localhost:5672".to_string());
        let _ = AMQPPool::new(config.clone()).unwrap();
        ut_info!("Success.");
    }
}
