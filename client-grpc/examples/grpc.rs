//! gRPC client implementation

use std::env;
#[allow(unused_qualifications, missing_docs)]
use svc_compliance_client_grpc::client::{
    compliance_rpc_client::ComplianceRpcClient, QueryIsReady,
};

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
    let grpc_endpoint = get_grpc_endpoint();

    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        grpc_endpoint
    );

    let mut client = ComplianceRpcClient::connect(grpc_endpoint).await?;

    println!("Client created");

    let response = client
        .is_ready(tonic::Request::new(QueryIsReady {}))
        .await?;

    println!("is_ready RESPONSE={:?}", response.into_inner());

    let response = client
        .submit_flight_plan(tonic::Request::new(
            svc_compliance_client_grpc::client::FlightPlanRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            },
        ))
        .await?;
    println!("submit_flight_plan RESPONSE={:?}", response.into_inner());

    let response = client
        .request_flight_release(tonic::Request::new(
            svc_compliance_client_grpc::client::FlightReleaseRequest {
                flight_plan_id: "".to_string(),
                data: "".to_string(),
            },
        ))
        .await?;
    println!(
        "request_flight_release RESPONSE={:?}",
        response.into_inner()
    );

    Ok(())
}
