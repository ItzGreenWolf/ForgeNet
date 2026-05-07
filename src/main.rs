// Update main.rs snippet for wallet integration
// Add to existing main.rs or new module
use crate::wallet::Wallet;

// In RPC or CLI
fn create_wallet() {
    let mut blacklist = HashSet::new();
    let wallet = Wallet::generate();
    if Wallet::is_blacklisted(&wallet.get_public_address(), &mut blacklist) {
        // regenerate
    } else {
        Wallet::blacklist_address(wallet.get_public_address(), &mut blacklist);
    }
    println!("New Wallet Address: {}", wallet.get_public_address());
}
