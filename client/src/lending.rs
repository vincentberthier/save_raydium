use solana_sdk::pubkey::Pubkey;
use spl_token_2022::{extension::StateWithExtensions, state};
use tracing::{debug, instrument};

use crate::transaction::get_rpc;

/// Get the mint of an ATA
///
/// # Parameters
/// * `account` Account (address) to read.
///
/// # Returns
/// If the account does not exist, or is not of a valid type,
/// `None` will be returned.
#[instrument]
pub async fn get_mint_address(account: &Pubkey) -> Option<Pubkey> {
    debug!("getting mint address associated to account");
    let rpc = get_rpc();

    let account = rpc.get_account(account).await.ok()?;
    Some(
        StateWithExtensions::<state::Account>::unpack(&account.data)
            .ok()?
            .base
            .mint,
    )
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use std::assert_matches::assert_matches;

    use solana_sdk::pubkey;
    use test_log::test;
    use tracing::info;

    use crate::config::set_config;

    use super::*;
    type TestResult = core::result::Result<(), Box<dyn core::error::Error>>;

    #[test(tokio::test)]
    async fn get_mint() -> TestResult {
        // Given
        const BSOL_MINT: Pubkey = pubkey!("bSo13r4TkiE4KumL71LsHTPpL2euBYLFx6h9HP3piy1");
        const BSOL_ATA: Pubkey = pubkey!("FtyYfaF1w7qZVHjLwB9mb4mhSjiFh1Fc1dWbQyrhN6dT");

        set_config()?;

        // When
        let mint = get_mint_address(&BSOL_ATA).await;

        // Then
        info!("mint: {mint:?}");
        assert_matches!(mint, Some(key) if key == BSOL_MINT);

        Ok(())
    }
}
