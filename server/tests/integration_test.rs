//! Integration Tests
//!
fn get_log_string(function: &str, name: &str) -> String {
    cfg_if::cfg_if! {
        if #[cfg(feature = "us")] {
            let lang = "us";
        } else {
            let lang = "nl";
        }
    }

    #[cfg(feature = "stub_server")]
    return format!("({} MOCK)[{}] {} server.", function, lang, name);

    #[cfg(not(feature = "stub_server"))]
    return format!("({})[{}] {} server.", function, lang, name);
}

#[tokio::test]
async fn test_server_requests_and_logs() {
    use logtest::Logger;
    use svc_compliance::grpc::server::*;

    let name = "compliance";

    // Start the logger.
    let mut logger = Logger::start();

    //test_is_ready_request_logs
    {
        let imp = ServerImpl {
            mq_channel: None,
            region: Box::<svc_compliance::region::RegionImpl>::default(),
        };

        let result = imp.is_ready(tonic::Request::new(ReadyRequest {})).await;
        assert!(result.is_ok());
        let result: ReadyResponse = result.unwrap().into_inner();
        assert_eq!(result.ready, true);

        // Search for the expected log message
        let expected = get_log_string("is_ready", name);
        println!("expected message: {}", expected);
        assert!(logger.any(|log| {
            if log.target().contains("app::") {
                println!("{}", log.target());
                let message = log.args();
                println!("{:?}", message);
                log.args() == expected
            } else {
                false
            }
        }));
    }
}
