use crate::errors::VoteMarketManagerError;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;

pub fn get_priority_fee() -> Result<f64, Box<dyn std::error::Error>> {
    let rpc_url = env::var("RPC_URL").unwrap().to_string();
    if !rpc_url.contains("helius") {
        return Ok(0.0);
    }
    let client = Client::new();
    let response = client
        .post(rpc_url)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getPriorityFeeEstimate",
            "params": [ {
                "accountKeys": [gauge_state::id().to_string(),"JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"],
                "options": {
                    "priority_level": "MEDIUM",
                }
            } ]
        }))
        .send()?;
    let json_response: serde_json::Value = response.json()?;
    json_response["result"]["priorityFeeEstimate"]
        .as_f64()
        .ok_or(VoteMarketManagerError::PriorityFeeNotInResult.into())
}
