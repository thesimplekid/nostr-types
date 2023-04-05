// TEMPORARILY
#![allow(clippy::uninlined_format_args)]

use nostr_types_lib::PrivateKey;
use zeroize::Zeroize;

// The zeroize in here is really silly because we print it.
fn main() {
    let mut hex = rpassword::prompt_password("Private key hex: ").unwrap();
    let mut private_key = PrivateKey::try_from_hex_string(&hex).unwrap();
    hex.zeroize();
    let mut bech32 = private_key.as_bech32_string();
    println!("{}", bech32);
    bech32.zeroize();
}
