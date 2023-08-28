//! gRPC client implementation
//!
use lib_common::grpc::get_endpoint_from_env;
use svc_compliance_client_grpc::prelude::{compliance::*, *};

/// Example svc-compliance-client-grpc
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    let client = ComplianceClient::new_client(&host, port, "compliance");
    println!("client created");
    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        client.get_address()
    );

    let response = client.is_ready(ReadyRequest {}).await?;

    println!("is_ready RESPONSE={:?}", response.into_inner());

    let response = client
        .submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        })
        .await?;
    println!("submit_flight_plan RESPONSE={:?}", response.into_inner());

    let response = client
        .request_flight_release(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        })
        .await?;
    println!(
        "request_flight_release RESPONSE={:?}",
        response.into_inner()
    );

    Ok(())
}
