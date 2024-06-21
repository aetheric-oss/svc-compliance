//! provides AMQP/RabbitMQ implementations for queuing layer

#[macro_use]
pub mod macros;
pub mod pool;
use crate::config::Config;
use lapin::{options::BasicPublishOptions, BasicProperties, Channel};

/// Name of the AMQP exchange for flightplan messages
pub const EXCHANGE_NAME_FLIGHTPLAN: &str = "flightplan";

/// Name of the AMQP queue for CARGO messages
pub const QUEUE_NAME_CARGO: &str = "cargo";

/// Routing key for CARGO messages
pub const ROUTING_KEY_CARGO: &str = "cargo";

/// Custom Error type for MQ errors
#[derive(thiserror::Error, Debug, Copy, Clone, PartialEq)]
pub enum AMQPError {
    /// Could Not Publish
    #[error("error: Could not publish to queue.")]
    CouldNotPublish,

    /// Could not connect to the AMQP pool.
    #[error("error: Could not connect to amqp pool.")]
    CouldNotConnect,

    /// Missing configuration
    #[error("error: Missing configuration for amqp pool connection.")]
    MissingConfiguration,

    /// Could not create channel
    #[error("error: Could not create channel.")]
    CouldNotCreateChannel,

    /// Could not declare queue
    #[error("error: Could not declare queue.")]
    CouldNotDeclareQueue,

    /// Could not declare exchange
    #[error("error: Could not declare exchange.")]
    CouldNotDeclareExchange,
}

/// Wrapper struct to allow unit testing on un-connected amqp_channel
#[derive(Debug)]
pub struct AMQPChannel {
    /// The Channel if available
    pub channel: Option<Channel>,
}

/// Stub function to initialize the AMQP connection. Does nothing.
#[cfg(feature = "stub_server")]
#[cfg(not(tarpaulin_include))]
// no_coverage: (Rnever) stub
pub async fn init_mq(_config: Config) -> Result<(), AMQPError> {
    Ok(())
}

cfg_if::cfg_if! {
    if #[cfg(feature = "test_util")] {
        impl AMQPChannel {
            /// Wrapper function for Channel basic_publish
            pub async fn basic_publish(
                &self,
                exchange: &str,
                routing_key: &str,
                options: BasicPublishOptions,
                payload: &[u8],
                properties: BasicProperties,
            ) -> Result<(), AMQPError> {
                let Some(channel) = &self.channel else {
                    amqp_warn!("No channel set AMQPChannel.");
                    return Ok(())
                };

                channel
                    .basic_publish(exchange, routing_key, options, payload, properties)
                    .await
                    .map_err(|e| {
                        amqp_error!("Could not publish: {}", e);
                        AMQPError::CouldNotPublish
                    })?;

                Ok(())
            }
        }
    } else {
        use lapin::publisher_confirm::PublisherConfirm;
        impl AMQPChannel {
            /// Wrapper function for Channel basic_publish
            pub async fn basic_publish(&self, exchange: &str, routing_key: &str, options: BasicPublishOptions, payload: &[u8], properties: BasicProperties) -> lapin::Result<PublisherConfirm> {
                let channel = self.channel.as_ref().ok_or_else(|| {
                    amqp_error!("No channel set AMQPChannel.");
                    lapin::Error::InvalidChannelState(lapin::ChannelState::Error)
                })?;

                channel.basic_publish(exchange, routing_key, options, payload, properties).await
            }
        }
    }
}

/// Initializes the AMQP connection. Creates the flightplan exchange and queues.
#[cfg(not(feature = "stub_server"))]
#[cfg(not(tarpaulin_include))]
// no_coverage: (Rnever) not unit testable, only integration tests
pub async fn init_mq(config: Config) -> Result<Channel, AMQPError> {
    // Establish connection to RabbitMQ node
    let pool = pool::AMQPPool::new(config.clone())?;
    let amqp_connection = pool.get_connection().await?;

    // Create channel
    amqp_info!("Creating channel...");
    let amqp_channel = amqp_connection.create_channel().await.map_err(|e| {
        amqp_error!("Could not create channel: {}", e);
        AMQPError::CouldNotCreateChannel
    })?;

    // Declare CARGO Queue
    amqp_info!("Creating '{QUEUE_NAME_CARGO}' queue...");
    let _ = amqp_channel
        .queue_declare(
            QUEUE_NAME_CARGO,
            lapin::options::QueueDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| {
            amqp_error!("Could not declare queue '{QUEUE_NAME_CARGO}': {}", e);
            AMQPError::CouldNotDeclareQueue
        })?;

    //
    // Declare a topic exchange
    //
    amqp_info!("Declaring exchange '{EXCHANGE_NAME_FLIGHTPLAN}'...");
    amqp_channel
        .exchange_declare(
            EXCHANGE_NAME_FLIGHTPLAN,
            lapin::ExchangeKind::Topic,
            lapin::options::ExchangeDeclareOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| {
            amqp_error!(
                "Could not declare exchange '{EXCHANGE_NAME_FLIGHTPLAN}': {}",
                e
            );
            AMQPError::CouldNotDeclareExchange
        })?;

    //
    // Bind the CARGO queue to the exchange
    //
    amqp_info!("Binding queue '{QUEUE_NAME_CARGO}' to exchange '{EXCHANGE_NAME_FLIGHTPLAN}'...");
    amqp_channel
        .queue_bind(
            QUEUE_NAME_CARGO,
            EXCHANGE_NAME_FLIGHTPLAN,
            ROUTING_KEY_CARGO,
            lapin::options::QueueBindOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .map_err(|e| {
            amqp_error!(
                "Could not bind queue '{QUEUE_NAME_CARGO}' to exchange: {}",
                e
            );

            AMQPError::CouldNotDeclareExchange
        })?;

    Ok(amqp_channel)
}
