#![doc = include_str!("../README.md")]

mod config;
mod grpc;
mod region;

use dotenv::dotenv;
use log::{error, info};

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    info!("(svc-compliance) server startup.");

    //initialize dotenv library which reads .env file
    dotenv().ok();

    // initialize config from .env file
    let config = config::Config::from_env().unwrap_or_default();

    // initialize logger
    let log_cfg = config.log_config.as_str();
    if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
        error!("(logger) could not parse {}. {}", log_cfg, e);
        panic!();
    }

    // start gRPC server
    let _ = tokio::spawn(grpc::server::server(config)).await;

    info!("(svc-compliance) server shutdown.");
    Ok(())
}
