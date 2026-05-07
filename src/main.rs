use warp::Filter;
use std::sync::{Arc, Mutex};
use serde::{Deserialize, Serialize};
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct RigScore {
    cpu: f64,
    gpu: f64,
    ram_gb: f64,
    bw_mbps: f64,
    total: f64,
}

#[derive(Debug, Serialize)]
struct RpcResponse {
    status: String,
    data: serde_json::Value,
}

// Simple in-memory blockchain state
struct Blockchain {
    // ... existing from before
}

#[tokio::main]
async fn main() {
    let blockchain = Arc::new(Mutex::new(Blockchain::new()));

    // JSON-RPC routes
    let get_rig_score = warp::path("rpc")
        .and(warp::path("get_rig_score"))
        .and(warp::query::<std::collections::HashMap<String, String>>())
        .map(|params: std::collections::HashMap<String, String>| {
            let cpu: f64 = params.get("cpu").and_then(|v| v.parse().ok()).unwrap_or(4.0);
            // similar for others
            let score = calculate_rig_score(cpu, 0.0, 8.0, 100.0);
            warp::reply::json(&RpcResponse { status: "success".to_string(), data: serde_json::json!({"score": score}) })
        });

    let routes = get_rig_score.or(/* other endpoints */);

    println!("ForgeNet JSON-RPC server running on http://127.0.0.1:3030");
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn calculate_rig_score(cpu: f64, gpu: f64, ram_gb: f64, bw_mbps: f64) -> f64 {
    cpu * 0.25 + gpu * 0.35 + ram_gb * 0.15 + bw_mbps * 0.25
}