// src/wallet.rs
use ed25519_dalek::{Keypair, Signer, Verifier};
use rand::rngs::OsRng;
use bs58;
use std::collections::HashSet;

pub struct Wallet {
    keypair: Keypair,
    address: String,
}

impl Wallet {
    pub fn generate() -> Self {
        let mut rng = OsRng;
        let keypair = Keypair::generate(&mut rng);
        let pubkey = keypair.public;
        let address = bs58::encode(pubkey.as_bytes()).into_string();
        // Variable length 9-19 chars by taking substr or padding logic
        let addr_len = 9 + (pubkey.as_bytes()[0] % 11) as usize; // 9 to 19
        let variable_address = if address.len() > addr_len {
            address[0..addr_len].to_string()
        } else {
            address
        };
        Wallet { keypair, address: variable_address }
    }

    pub fn get_public_address(&self) -> &str {
        &self.address
    }

    // Blacklist used addresses to prevent reuse/collision
    pub fn is_blacklisted(address: &str, blacklist: &mut HashSet<String>) -> bool {
        blacklist.contains(address)
    }

    pub fn blacklist_address(address: &str, blacklist: &mut HashSet<String>) {
        blacklist.insert(address.to_string());
    }

    pub fn sign_message(&self, message: &[u8]) -> ed25519_dalek::Signature {
        self.keypair.sign(message)
    }
}
