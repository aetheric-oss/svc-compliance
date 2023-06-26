//! gRPC client implementation
//!
use lib_common::grpc::get_endpoint_from_env;
use svc_compliance_client_grpc::client::{
    Coordinate, CoordinateFilter, FlightPlanRequest, FlightReleaseRequest, ReadyRequest,
    RestrictionsRequest, RpcServiceClient, WaypointsRequest,
};
use svc_compliance_client_grpc::service::Client as ServiceClient;
use svc_compliance_client_grpc::{Client, GrpcClient};
use tonic::transport::Channel;

/// Example svc-compliance-client-grpc
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    let connection = GrpcClient::<RpcServiceClient<Channel>>::new_client(&host, port, "compliance");
    println!("Connection created");
    println!(
        "NOTE: Ensure the server is running on {} or this example will fail.",
        connection.get_address()
    );

    let filter = CoordinateFilter {
        min: Some(Coordinate {
            latitude: 30.0,
            longitude: -105.0,
        }),
        max: Some(Coordinate {
            latitude: 35.0,
            longitude: -100.0,
        }),
    };

    let response = connection.is_ready(ReadyRequest {}).await?;

    println!("is_ready RESPONSE={:?}", response.into_inner());

    let response = connection
        .submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        })
        .await?;
    println!("submit_flight_plan RESPONSE={:?}", response.into_inner());

    let response = connection
        .request_flight_release(FlightReleaseRequest {
            flight_plan_id: "".to_string(),
            data: "".to_string(),
        })
        .await?;
    println!(
        "request_flight_release RESPONSE={:?}",
        response.into_inner()
    );

    let response = connection
        .request_waypoints(WaypointsRequest {
            filter: Some(filter),
        })
        .await?;
    println!("request_waypoints RESPONSE={:?}", response.into_inner());

    let response = connection
        .request_restrictions(RestrictionsRequest {
            filter: Some(filter),
        })
        .await?;

    println!("request_restrictions RESPONSE={:?}", response.into_inner());

    Ok(())
}
