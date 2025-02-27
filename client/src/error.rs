use std::io;

use derive_more::derive::{Display, From};
use solana_client::{pubsub_client::PubsubClientError, rpc_request::RpcError};
use solana_rpc_client_api::client_error;
use solana_sdk::{instruction::InstructionError, transaction::TransactionError};

pub type Result<T> = core::result::Result<T, Error>;

/// Errors of the Save / Raydium application.
#[derive(Debug, Display, From)]
#[display("{_variant}")]
pub enum Error {
    Misc,
    #[display("invalid keypair")]
    Keypair,
    #[display("could not get the pubsub client: {_0}")]
    #[from]
    Pubsub(PubsubClientError),
    /// A `RpcRequestError` or `ForUser` RPC error.
    #[from]
    #[display("error in the Solana RPC: {}", _0)]
    Rpc(RpcError),
    /// An error in the Solana Client
    #[from]
    #[display("error in the Solana Client: {}", _0)]
    SolanaClient(client_error::Error),
    /// Error happened while handling a specific instruction.
    #[from]
    #[display("error in an instruction: {}\n{}", error, logs.join("\n"))]
    SolanaInstruction {
        error: InstructionError,
        logs: Vec<String>,
    },
    /// Error happened at the transaction level.
    #[from]
    #[display("transaction error: {}", _0)]
    SolanaTransaction(TransactionError),
    /// Error while serializing or deserializing binary data with Borsh.
    #[from]
    #[display("Borsh could not (de)serialize data: {}", _0)]
    BorshSerialization(io::Error),
    /// An RPC error that is not handled by a more precise error.
    #[display("misc Solana RPC error: {}", _0)]
    RpcMisc(String),
    /// The RPC failed to parse an account.
    #[display("RPC failed to parse data: {}", _0)]
    RpcParse(String),
}

impl core::error::Error for Error {}
