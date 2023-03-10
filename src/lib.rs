pub mod entry_points;
pub mod msgs;
pub mod errors;

// Version info
pub const CONTRACT_NAME: &str = "crates.io:cw-boolean-contract";
pub const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// Reply ID
pub const REPLY_CRONCAT_TASK_CREATION: u64 = 0;
