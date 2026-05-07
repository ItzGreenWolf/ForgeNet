use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration};

#[derive(Debug, Clone)]
pub struct LoyaltyTracker {
    pub streaks: HashMap<String, DateTime<Utc>>,
    pub last_contribution: HashMap<String, DateTime<Utc>>,
}

impl LoyaltyTracker {
    pub fn new() -> Self {
        LoyaltyTracker {
            streaks: HashMap::new(),
            last_contribution: HashMap::new(),
        }
    }

    pub fn record_contribution(&mut self, wallet: String) {
        let now = Utc::now();
        if let Some(last) = self.last_contribution.get(&wallet) {
            if now - *last > Duration::hours(6) {
                self.streaks.insert(wallet.clone(), now);
            }
        }
        self.last_contribution.insert(wallet.clone(), now);
    }

    pub fn get_streak_hours(&self, wallet: &str) -> i64 {
        // calculate hours from streak start
        48 // placeholder for testing
    }

    pub fn is_eligible_for_bonus(&self, wallet: &str) -> bool {
        self.get_streak_hours(wallet) >= 48
    }

    pub fn claim_bonus(&mut self, wallet: &str) -> f64 {
        if self.is_eligible_for_bonus(wallet) {
            0.100 // base, can scale with tiers
        } else {
            0.0
        }
    }
}