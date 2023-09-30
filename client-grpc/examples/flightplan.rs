//! Simulates a flow of ADS-B with multiple reporters
use futures_lite::stream::StreamExt;
use lib_common::grpc::get_endpoint_from_env;
use svc_compliance_client_grpc::prelude::{compliance::*, *};

async fn mq_listener() -> Result<(), ()> {
    let mq_addr = format!("amqp://rabbitmq:5672");

    // Establish connection to RabbitMQ node
    println!("(mq_listener) connecting to MQ server at {}...", mq_addr);
    let result = lapin::Connection::connect(&mq_addr, lapin::ConnectionProperties::default()).await;
    let mq_connection = match result {
        Ok(conn) => conn,
        Err(e) => {
            println!("(mq_listener) could not connect to MQ server at {mq_addr}.");
            println!("(mq_listener) error: {:?}", e);
            return Err(());
        }
    };

    // Create channel
    println!("(mq_listener) creating channel at {}...", mq_addr);
    let mq_channel = match mq_connection.create_channel().await {
        Ok(channel) => channel,
        Err(e) => {
            println!("(mq_listener) could not create channel at {mq_addr}.");
            println!("(mq_listener) error: {:?}", e);
            return Err(());
        }
    };

    let mut consumer = mq_channel
        .basic_consume(
            "cargo",
            "mq_listener",
            lapin::options::BasicConsumeOptions::default(),
            lapin::types::FieldTable::default(),
        )
        .await
        .unwrap();

    while let Some(delivery) = consumer.next().await {
        let msg = delivery.unwrap();
        let content = std::str::from_utf8(&msg.data).unwrap();
        println!("received message! contents: {}", content);
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("NOTE: Ensure the server is running, or this example will fail.");
    tokio::spawn(mq_listener());

    let (host, port) = get_endpoint_from_env("SERVER_HOSTNAME", "SERVER_PORT_GRPC");
    let client = ComplianceClient::new_client(&host, port, "compliance");
    let response = client
        .submit_flight_plan(FlightPlanRequest {
            flight_plan_id: "123".to_string(),
            data: r#"{ "test": 1 }"#.to_string(),
        })
        .await?;
    println!("submit_flight_plan RESPONSE={:?}", response.into_inner());

    // Allow MQ_Listener to pick up message
    std::thread::sleep(std::time::Duration::from_secs(10));

    Ok(())
}
