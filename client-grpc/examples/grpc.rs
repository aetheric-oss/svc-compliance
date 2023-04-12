//! gRPC client implementation

use client::rpc_service_client::RpcServiceClient;
use client::FlightPlanRequest;
use client::FlightReleaseRequest;
use client::ReadyRequest;
use client::RestrictionsRequest;
use client::WaypointsRequest;
use client::{Coordinate, CoordinateFilter};
use dotenv::dotenv;
use std::env;
#[allow(unused_qualifications, missing_docs)]
use svc_compliance_client_grpc::client;

/// Provide endpoint url to use
pub fn get_grpc_endpoint() -> String {
    //parse socket address from env variable or take default value
    let address = match env::var("SERVER_HOSTNAME") {
        Ok(val) => val,
        Err(_) => "localhost".to_string(), // default value
    };

    let port = match env::var("SERVER_PORT_GRPC") {
        Ok(val) => val,
        Err(_) => "50051".to_string(), // default value
    };

    format!("http://{}:{}", address, port)
}

/// Example svc-compliance-client-grpc
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let grpc_endpoint = get_grpc_endpoint();

    let region: String = match env::var("REGION_CODE") {
        Ok(val) => val,
        Err(_) => "nl".to_string(), // default value
    };

    println!("REGION_CODE={}", region);
    let filter = match region.as_str() {
        "nl" => CoordinateFilter {
            min: Some(Coordinate {
                latitude: 52.20,
                longitude: 4.4,
            }),
            max: Some(Coordinate {
                latitude: 53.4,
                longitude: 5.3,
            }),
        },
        "us" => CoordinateFilter {
            min: Some(Coordinate {
                latitude: 30.0,
                longitude: -105.0,
            }),
            max: Some(Coordinate {
                latitude: 35.0,
                longitude: -100.0,
            }),
        },
        _ => panic!(),
    };

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    let mut client = RpcServiceClient::connect(grpc_endpoint).await?;

    println!("Client created");

    let response = client
        .is_ready(tonic::Request::new(ReadyRequest {}))
        .await?;

    println!("is_ready RESPONSE={:?}", response.into_inner());

    let response = client
        .submit_flight_plan(tonic::Request::new(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        }))
        .await?;
    println!("submit_flight_plan RESPONSE={:?}", response.into_inner());

    let response = client
        .request_flight_release(tonic::Request::new(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        }))
        .await?;
    println!(
        "request_flight_release RESPONSE={:?}",
        response.into_inner()
    );

    let response = client
        .request_waypoints(tonic::Request::new(WaypointsRequest {
            filter: Some(filter),
        }))
        .await?;
    println!("request_waypoints RESPONSE={:?}", response.into_inner());

    let response = client
        .request_restrictions(tonic::Request::new(RestrictionsRequest {
            filter: Some(filter),
        }))
        .await?;

    println!("request_restrictions RESPONSE={:?}", response.into_inner());

    Ok(())
}
