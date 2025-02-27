use solana_client::{
    nonblocking::{pubsub_client::PubsubClient, rpc_client::RpcClient},
    rpc_request::{RpcError, RpcResponseErrorData},
    rpc_response::RpcSimulateTransactionResult,
};
use solana_rpc_client_api::client_error::{self, ErrorKind};
use solana_sdk::{
    commitment_config::{CommitmentConfig, CommitmentLevel},
    hash::Hash,
    instruction::Instruction,
    signature::{Keypair, Signature},
    signer::Signer,
    transaction::{Transaction, TransactionError},
};
use tokio::select;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::{
    config::{COMMITMENT_LEVEL, RPC_HTTP, RPC_WS, TRX_PAYER},
    error::{Error, Result},
};

/// Packages instructions into a transaction and executes it.
///
/// * `instructions` - Instructions to execute in the transaction,
///
/// # Errors
/// If the transaction fails to execute.
#[expect(clippy::expect_used)]
#[instrument(skip_all)]
pub async fn execute_instructions(instructions: &[Instruction]) -> Result<Signature> {
    debug!("executing transaction");
    let rpc = get_rpc();

    let payer = Keypair::from_bytes(TRX_PAYER.get().expect("trx payer is not set"))
        .map_err(|_err| Error::Keypair)?;
    let block = get_blockhash(&rpc).await?;
    let trx =
        Transaction::new_signed_with_payer(instructions, Some(&payer.pubkey()), &[&payer], block);

    trace!(
        signature = ?trx.signatures.first(),
        "transaction was created, sending it for execution",
    );
    let sig = rpc
        .send_and_confirm_transaction(&trx)
        .await
        .map_err(process_rpc_error)?;
    info!(" Transaction has passed preflight, waiting for confirmation");
    wait_for_finalization(&rpc, &sig).await?;

    Ok(sig)
}

/// Given a transaction’s signature, waits for its finalization on the blockchain.
///
/// # Parameters
/// * `rpc` - The RPC client,
/// * `sig` - Signature of the transaction to wait for.
///
/// # Errors
/// If the transaction fails to be finalized.
async fn wait_for_finalization(rpc: &RpcClient, sig: &Signature) -> Result<()> {
    let mut confirmed = false;

    loop {
        select! {
            res = rpc.poll_for_signature_with_commitment(sig, CommitmentConfig { commitment: CommitmentLevel::Confirmed }) => {
                match res {
                    Ok(()) => if !confirmed { info!(" Transaction has been confirmed, waiting for finalization"); confirmed = true; },
                    Err(err) => {
                        error!(" Transaction has not been finalized successfully");
                        return Err(process_rpc_error(err));
                    }
                }
            }
            res = rpc.poll_for_signature_with_commitment(sig, CommitmentConfig { commitment: CommitmentLevel::Finalized }) => {
                match res {
                    Ok(()) => { info!(" Transaction has been finalized"); break; }
                    Err(err) => {
                        error!(" Transaction has not been finalized successfully");
                        return Err(process_rpc_error(err));
                    }
                }
            }
        }
    }

    Ok(())
}

/// Get the Solana RPC client.
///
/// There is a rate limiter associated to it, to prevent too many instructions from being
/// sent to the blockchain through the Quick Node API.
#[expect(clippy::expect_used)]
#[instrument]
pub fn get_rpc() -> RpcClient {
    let url = RPC_HTTP
        .get()
        .cloned()
        .expect("The RPC_HTTP address was not set");

    debug!("getting RPC client at {url}");
    RpcClient::new_with_commitment(
        url,
        CommitmentConfig {
            commitment: COMMITMENT_LEVEL,
        },
    )
}

/// Get the client to subscribe to for event monitoring.
#[expect(clippy::expect_used)]
#[instrument]
pub async fn get_pubsub() -> Result<PubsubClient> {
    let url = RPC_WS
        .get()
        .cloned()
        .expect("The RPC_WS address was not set");
    debug!("getting pubsub client at {url}");
    PubsubClient::new(&url).await.map_err(Error::Pubsub)
}

async fn get_blockhash(rpc: &RpcClient) -> Result<Hash> {
    rpc.get_latest_blockhash().await.map_err(process_rpc_error)
}

/// Get a precise error from an `RpcError`
///
/// Deconstructing the error as much as possible to get to the root of the issue
/// and make the returned error as clear as possible. The most likely error
/// is one that happened during the simulation, that's why we're focusing on that
/// the others don't necessitate as much details so can be returned as is.
///
/// # Parameters
/// * `error` - The error produced by an RPC call.
#[instrument(skip_all)]
pub fn process_rpc_error(error: client_error::Error) -> Error {
    error!(%error, "Solana RPC error");
    let (error, logs) = match error.kind {
        ErrorKind::RpcError(RpcError::RpcResponseError {
            data:
                RpcResponseErrorData::SendTransactionPreflightFailure(RpcSimulateTransactionResult {
                    err: Some(trx_error),
                    logs,
                    ..
                }),
            ..
        }) => (trx_error, logs),
        ErrorKind::RpcError(RpcError::ParseError(err)) => return Error::RpcParse(err),
        ErrorKind::RpcError(err) => return Error::Rpc(err),
        err => return Error::RpcMisc(err.to_string()),
    };

    match error {
        TransactionError::InstructionError(_, instr_error) => Error::SolanaInstruction {
            error: instr_error,
            logs: logs.unwrap_or_default(),
        },
        error => Error::SolanaTransaction(error),
    }
}

#[cfg(test)]
#[cfg_attr(coverage_nightly, coverage(off))]
mod tests {

    use std::assert_matches::assert_matches;

    use solana_sdk::{signature::Keypair, signer::Signer};
    use test_log::test;

    use crate::config::{SOURCE, TARGET, set_config};

    use super::*;
    type Result<T> = core::result::Result<T, Box<dyn core::error::Error>>;

    #[test(tokio::test)]
    async fn get_latest_blockhash() -> Result<()> {
        // Given
        set_config()?;
        let rpc = get_rpc();

        // When
        let hash = get_blockhash(&rpc).await;

        // Then
        assert_matches!(hash, Ok(_hash), "{hash:?}");

        Ok(())
    }

    #[test(tokio::test)]
    async fn execute_transaction() -> Result<()> {
        // Given
        const LAMPORTS: u64 = 10;
        set_config()?;
        let source_pk = Keypair::from_bytes(SOURCE)?;
        let target = Keypair::from_bytes(TARGET)?.pubkey();
        let instruction =
            solana_sdk::system_instruction::transfer(&source_pk.pubkey(), &target, LAMPORTS);

        // When
        let res = execute_instructions(&[instruction]).await;

        // Then
        assert_matches!(res, Ok(_sig), "{res:?}");

        Ok(())
    }
}
