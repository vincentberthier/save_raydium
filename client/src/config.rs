use std::sync::OnceLock;

use solana_sdk::pubkey;
use solana_sdk::{commitment_config::CommitmentLevel, pubkey::Pubkey};

/// The keypair of the transaction payer.
pub static TRX_PAYER: OnceLock<[u8; 64]> = OnceLock::new();

/// The commitment level set for the RPC client
pub const COMMITMENT_LEVEL: CommitmentLevel = CommitmentLevel::Processed;

/// Address of the Solana RPC via HTTP.
pub static RPC_HTTP: OnceLock<String> = OnceLock::new();
/// Address of the Solana RPC via `WS`.
pub static RPC_WS: OnceLock<String> = OnceLock::new();

/// Wrapped Solana SPL token mint
pub const WSOL_MINT: Pubkey = pubkey!("So11111111111111111111111111111111111111112");
/// bSOL SPL tokens mint
pub const BSOL_MINT: Pubkey = pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1");

#[cfg(test)]
// 4vCvi2VqJbRSe33F9kfVQY6R8MX25eDK1f1TtuFngbwp
pub const SOURCE: &[u8; 64] = &[
    153, 169, 33, 203, 65, 237, 143, 120, 101, 196, 21, 52, 129, 16, 190, 228, 144, 255, 214, 140,
    104, 62, 163, 219, 170, 188, 135, 201, 214, 53, 120, 155, 58, 52, 84, 19, 58, 63, 157, 208,
    177, 90, 191, 85, 184, 176, 84, 9, 101, 26, 217, 116, 178, 202, 196, 191, 128, 102, 53, 45, 71,
    2, 144, 91,
];

#[cfg(test)]
// 5rfMKbh1EYjjmE86fxGfHTbU2ENm8w7nUYmGXd1gStnp
pub const TARGET: &[u8; 64] = &[
    53, 100, 38, 40, 243, 211, 25, 37, 144, 107, 36, 140, 140, 197, 128, 180, 20, 16, 173, 209, 63,
    156, 111, 137, 85, 135, 69, 130, 43, 38, 166, 245, 72, 39, 145, 250, 253, 19, 207, 237, 58, 50,
    49, 190, 197, 138, 64, 98, 168, 106, 171, 46, 62, 148, 200, 85, 72, 126, 19, 226, 16, 143, 218,
    229,
];

#[cfg(test)]
pub fn set_config() -> Result<(), Box<dyn core::error::Error>> {
    RPC_HTTP.set("https://api.devnet.solana.com".to_owned())?;
    RPC_WS.set("wss://api.devnet.solana.com/".to_owned())?;
    TRX_PAYER
        .set(*SOURCE)
        .map_err(|_err| "could not set TRX_PAYER")?;
    Ok(())
}
