pub mod chain_handlers;
pub mod static_handlers;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PingResponse {
    pub success: bool,
}
