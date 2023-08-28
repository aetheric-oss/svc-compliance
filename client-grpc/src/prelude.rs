//! Re-export of used objects

pub use super::client as compliance;
pub use super::service::Client as ComplianceServiceClient;
pub use compliance::ComplianceClient;

pub use lib_common::grpc::Client;
