//! Integration Tests

#[tokio::test]
async fn test_server_requests_and_logs() {
    use logtest::Logger;

    use svc_compliance::grpc::server::*;

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let imp = ServerImpl::default();
        let result = imp.is_ready(tonic::Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

        assert!(logger.any(|log| {
            let message = log.args();
            println!("{:?}", message);
            cfg_if::cfg_if! {
                if #[cfg(feature = "nl")] {
                    log.args() == format!("([nl] is_ready MOCK) {} server.", "compliance")
                } else {
                    log.args() == format!("([us] is_ready MOCK) {} server.", "compliance")
                }
            }
        }));
    }
}
