//! <center>
//! <img src="https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png" style="height:250px" />
//! </center>
//! <div align="center">
//!     <a href="https://github.com/Arrow-air/svc-compliance/releases">
//!         <img src="https://img.shields.io/github/v/release/Arrow-air/svc-compliance?sort=semver&color=green" alt="GitHub stable release (latest by date)">
//!     </a>
//!     <a href="https://github.com/Arrow-air/svc-compliance/releases">
//!         <img src="https://img.shields.io/github/v/release/Arrow-air/svc-compliance?include_prereleases" alt="GitHub release (latest by date including pre-releases)">
//!     </a>
//!     <a href="https://github.com/Arrow-air/svc-compliance/tree/main">
//!         <img src="https://github.com/arrow-air/svc-compliance/actions/workflows/rust_ci.yml/badge.svg?branch=main" alt="Rust Checks">
//!     </a>
//!     <a href="https://discord.com/invite/arrow">
//!         <img src="https://img.shields.io/discord/853833144037277726?style=plastic" alt="Arrow DAO Discord">
//!     </a>
//!     <br><br>
//! </div>
//!
//! This service is responsible for all communication with regional aviation authorities.

mod region_interface;
mod regions;

///module svc_storage generated from svc-storage.proto
pub mod svc_compliance {
    #![allow(unused_qualifications, missing_docs)]
    include!("grpc.rs");
}

use crate::region_interface::RegionInterface;
use crate::svc_compliance::{
    FlightPlanRequest, FlightPlanResponse, FlightReleaseRequest, FlightReleaseResponse,
};
use dotenv::dotenv;
use svc_compliance::compliance_rpc_server::{ComplianceRpc, ComplianceRpcServer};
use svc_compliance::{QueryIsReady, ReadyResponse};
use tonic::{transport::Server, Request, Response, Status};

use log::error;

///Implementation of gRPC endpoints
#[derive(Debug, Default, Copy, Clone)]
pub struct ComplianceImpl {}

#[tonic::async_trait]
impl ComplianceRpc for ComplianceImpl {
    /// Returns ready:true when service is available
    async fn is_ready(
        &self,
        _request: Request<QueryIsReady>,
    ) -> Result<Response<ReadyResponse>, Status> {
        let response = ReadyResponse { ready: true };
        Ok(Response::new(response))
    }

    async fn submit_flight_plan(
        &self,
        request: Request<FlightPlanRequest>,
    ) -> Result<Response<FlightPlanResponse>, Status> {
        match get_region_impl() {
            Ok(region) => region.submit_flight_plan(request),
            Err(_) => Err(Status::internal("Failed to submit flight plan.")),
        }
    }

    async fn request_flight_release(
        &self,
        request: Request<FlightReleaseRequest>,
    ) -> Result<Response<FlightReleaseResponse>, Status> {
        match get_region_impl() {
            Ok(region) => region.request_flight_release(request),
            Err(_) => Err(Status::internal("Failed to request flight release.")),
        }
    }
}

///Returns region implementation based on REGION_CODE environment variable
fn get_region_impl() -> Result<Box<dyn RegionInterface>, ()> {
    let Ok(region) = std::env::var("REGION_CODE") else {
        error!("REGION_CODE environment variable is not set");
        return Err(())
    };

    match region.as_str() {
        "us" => Ok(Box::new(regions::us::USImpl {})),
        "nl" => Ok(Box::new(regions::nl::NLImpl {})),
        _ => {
            error!("Unknown region: {}", region);
            Err(())
        }
    }
}

///Main entry point: starts gRPC Server on specified address and port
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    //initialize dotenv library which reads .env file
    dotenv().ok();

    //initialize logger
    let log_cfg: &str = "log4rs.yaml";
    if let Err(e) = log4rs::init_file(log_cfg, Default::default()) {
        println!("(logger) could not parse {}. {}", log_cfg, e);
        panic!();
    }

    // check region implementation and panic if region code is unknown
    if get_region_impl().is_err() {
        panic!();
    }

    // GRPC Server
    let grpc_port = std::env::var("DOCKER_PORT_GRPC")
        .unwrap_or_else(|_| "50051".to_string())
        .parse::<u16>()
        .unwrap_or(50051);

    let full_grpc_addr = format!("[::]:{}", grpc_port).parse()?;

    let (mut health_reporter, health_service) = tonic_health::server::health_reporter();
    let imp = ComplianceImpl::default();
    health_reporter
        .set_serving::<ComplianceRpcServer<ComplianceImpl>>()
        .await;

    //start server
    println!("Starting gRPC server at: {}", full_grpc_addr);
    Server::builder()
        .add_service(health_service)
        .add_service(ComplianceRpcServer::new(imp))
        .serve(full_grpc_addr)
        .await?;
    println!("gRPC server running at: {}", full_grpc_addr);

    Ok(())
}
