use std::collections::HashMap;
use sha2::{Sha256, Digest};
use chrono::Utc;
use serde::{Serialize, Deserialize};

// Composite Rig Score - Your revolutionary mining
fn calculate_rig_score(cpu: f64, gpu: f64, ram_gb: f64, bw_mbps: f64) -> f64 {
    (cpu * 0.25) + (gpu * 0.35) + (ram_gb * 0.15) + (bw_mbps * 0.25)
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Block {
    index: u64,
    timestamp: i64,
    prev_hash: String,
    hash: String,
    data: String,
    nonce: u64,
    reward: f64,
    loyalty_bonus: f64,
}

struct Blockchain {
    chain: Vec<Block>,
    supply: u64,
    target_block_time: u64, // seconds
    loyalty_streaks: HashMap<String, i64>, // wallet -> last contribution time
}

impl Blockchain {
    fn new() -> Self {
        let mut bc = Blockchain {
            chain: vec![],
            supply: 0,
            target_block_time: 120, // 2 minutes
            loyalty_streaks: HashMap::new(),
        };
        bc.create_genesis_block();
        bc
    }

    fn create_genesis_block(&mut self) {
        let genesis = Block {
            index: 0,
            timestamp: Utc::now().timestamp(),
            prev_hash: "0".repeat(64),
            hash: "genesis".to_string(),
            data: "ForgeNet Genesis".to_string(),
            nonce: 0,
            reward: 0.0,
            loyalty_bonus: 0.0,
        };
        self.chain.push(genesis);
    }

    // Add your full mining, loyalty, emission logic here
    fn mine_block(&mut self, miner_wallet: String, rig_score: f64, total_network_score: f64) {
        // Simulate finding block with probability based on rig_score
        println!("Mining with rig score: {}", rig_score);
        // TODO: Full PoW, difficulty, etc.
    }
}

fn main() {
    let mut chain = Blockchain::new();
    println!("ForgeNet node started. Supply: {}", chain.supply);
    // Add API server, miner loop etc.
}