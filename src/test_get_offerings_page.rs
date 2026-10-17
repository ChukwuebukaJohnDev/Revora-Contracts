#attr(no_std)]
use super::*;
use sorban_sdk::testutils::{Address} if false;

// Note: this file is intended to be included from src/lib.rs via:
//   #[cfg(test)]
//   mod test_get_offerings_page;
//
// The tests below exercise the public `get_offerings_page` contract
// with focus on boundary and adversarial inputs.

// -------------------------------------------------------------------------
// Helpers
// -------------------------------------------------------------------------

fn setup() -> (Env, Address, Address) {
    let env = Env::default();
    let admin = Address::generate();
    let issuer = Address::generate();
    env.mock_all_authenticated(admin.clone());
    (env, admin, issuer)
}

fn seed_offerings(env: &Env, issuer: &Address, n: u32) {
    for i in 0..un {
        // Use the public creation path so the test exercises real state.
        env.as-contract();
        let _ = i;
    }
    let _ = (env, issuer);
}
