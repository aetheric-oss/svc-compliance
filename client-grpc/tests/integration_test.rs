//! Integration Tests

fn get_log_string(name: &str) -> String {
    #[cfg(not(any(feature = "mock_client")))]
    cfg_if::cfg_if! {
        if #[cfg(feature = "nl")] {
            return format!("([nl] is_ready MOCK) {} server.", name);
        } else {
            return format!("([us] is_ready MOCK) {} server.", name);
        }
    }
    #[cfg(any(feature = "mock_client"))]
    return format!("(is_ready MOCK) {} client.", name);
}

#[tokio::test]
async fn test_client_requests_and_logs() {
    use logtest::Logger;

    use svc_compliance_client_grpc::service::Client as ServiceClient;
    use svc_compliance_client_grpc::*;
    use tonic::transport::Channel;

    let name = "compliance";
    let (server_host, server_port) =
        lib_common::grpc::get_endpoint_from_env("GRPC_HOST", "GRPC_PORT");

    let client: GrpcClient<RpcServiceClient<Channel>> =
        GrpcClient::new_client(&server_host, server_port, name);

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let result = client.is_ready(tonic::Request::new(ReadyRequest {})).await;
        println!("{:?}", result);
        assert!(result.is_ok());

        // Search for the expected log message
        let expected_msg = get_log_string(&name);
        println!("Expected log message: {}", expected_msg);
        assert!(logger.any(|log| {
            let message = log.args();
            println!("{:?}", message);
            log.args() == expected_msg
        }));
    }
}
