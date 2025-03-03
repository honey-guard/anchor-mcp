pub mod utilities;
pub mod types;
pub mod prompts;
pub mod tools;

const JSONRPC_VERSION: &str = "2.0";
const PROTOCOL_VERSION: &str = "2024-11-05";
const SERVER_NAME: &str = "anchor-mcp";
const SERVER_VERSION: &str = env!("CARGO_PKG_VERSION");
