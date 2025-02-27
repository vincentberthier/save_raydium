use anchor_client::solana_sdk::{pubkey::Pubkey, signature::Keypair, system_program, sysvar};
use anchor_client::{Client, ClientError, Program};
use solana_client::rpc_request::{RpcError, RpcResponseErrorData};
use solana_client::rpc_response::RpcSimulateTransactionResult;
use solana_rpc_client_api::client_error::{self, ErrorKind};
use solana_sdk::signer::Signer;
use solana_sdk::transaction::TransactionError;
use solana_sdk::{pubkey, system_instruction};
use std::rc::Rc;
use tracing::{info, instrument};

use crate::error::{Error, Result};

const PROGRAM_ID: Pubkey = pubkey!("5Xs3m9xLbGFYY8C62PxuqAZjwmHnQuAzdjq6xtoKmVbF");

#[expect(clippy::unwrap_used)]
pub fn get_program(client: &Client<Rc<Keypair>>) -> Program<Rc<Keypair>> {
    client.program(PROGRAM_ID).unwrap()
}

#[expect(clippy::result_large_err, clippy::unwrap_used)]
pub fn init_lending_market(
    client: &Client<Rc<Keypair>>,
    owner: &Keypair,
    market: &Keypair,
) -> Result<()> {
    const QUOTE_CURRENCY: &[u8; 32] =
        b"USD\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0\0";
    let program = get_program(client);

    let market_authority =
        Pubkey::find_program_address(&[b"lma", market.pubkey().as_ref()], &PROGRAM_ID).0;
    info!("Market authority: {}", market_authority);

    let rent_exempt_balance = program
        .rpc()
        .get_minimum_balance_for_rent_exemption(size_of::<klend::state::LendingMarket>() + 8)
        .unwrap();

    // Create the account
    let create_account_ix = system_instruction::create_account(
        &owner.pubkey(),
        &market.pubkey(),
        rent_exempt_balance,
        size_of::<klend::state::LendingMarket>() as u64 + 8,
        &PROGRAM_ID,
    );

    let tx = program
        .request()
        .instruction(create_account_ix)
        .accounts(klend::accounts::InitLendingMarket {
            lending_market_owner: owner.pubkey(),
            lending_market: market.pubkey(),
            lending_market_authority: market_authority,
            system_program: system_program::ID,
            rent: sysvar::rent::ID,
        })
        .args(klend::instruction::InitLendingMarket {
            _quote_currency: *QUOTE_CURRENCY,
        })
        .signer(owner)
        .signer(&market)
        .send()
        .map_err(process_rpc_error)?;
    info!("Lending Market Initialized: {:?}", tx);

    Ok(())
}

pub fn update_lending_market(
    client: &Client<Rc<Keypair>>,
    wallet: &Keypair,
    lending_market: Pubkey,
    mode: u64,
    value: [u8; 72],
) {
    let program = get_program(client);
    let tx = program
        .request()
        .accounts(klend::accounts::UpdateLendingMarket {
            lending_market_owner: wallet.pubkey(),
            lending_market,
        })
        .args(klend::instruction::UpdateLendingMarket {
            _mode: mode,
            _value: value,
        })
        .signer(wallet)
        .send();
    info!("Lending Market Updated: {:?}", tx);
}

pub fn init_reserve(
    client: &Client<Rc<Keypair>>,
    wallet: &Keypair,
    lending_market: Pubkey,
    reserve_mint: Pubkey,
) {
    let program = get_program(client);
    let reserve = Keypair::new();

    let tx = program
        .request()
        .accounts(klend::accounts::InitReserve {
            lending_market_owner: wallet.pubkey(),
            lending_market,
            lending_market_authority: Pubkey::default(),
            reserve: reserve.pubkey(),
            reserve_liquidity_mint: reserve_mint,
            reserve_liquidity_supply: Pubkey::default(),
            fee_receiver: Pubkey::default(),
            reserve_collateral_mint: Pubkey::default(),
            reserve_collateral_supply: Pubkey::default(),
            rent: sysvar::rent::ID,
            token_program: spl_token_2022::ID,
            system_program: solana_sdk::system_program::ID,
        })
        .signer(wallet)
        .signer(&reserve)
        .send();
    info!("Reserve Initialized: {:?}", tx);
}

pub fn lend(client: &Client<Rc<Keypair>>, wallet: &Keypair, reserve: Pubkey, amount: u64) {
    let program = get_program(client);
    let tx = program
        .request()
        .accounts(klend::accounts::DepositReserveLiquidity {
            owner: wallet.pubkey(),
            reserve,
            lending_market: Pubkey::default(),
            lending_market_authority: Pubkey::default(),
            reserve_liquidity_supply: Pubkey::default(),
            reserve_collateral_mint: Pubkey::default(),
            user_source_liquidity: Pubkey::default(),
            user_destination_collateral: Pubkey::default(),
            token_program: system_program::ID,
        })
        .args(klend::instruction::DepositReserveLiquidity {
            _liquidity_amount: amount,
        })
        .signer(wallet)
        .send();
    info!("Lent: {:?}", tx);
}

pub fn borrow(
    client: &Client<Rc<Keypair>>,
    wallet: &Keypair,
    obligation: Pubkey,
    borrow_reserve: Pubkey,
    amount: u64,
) {
    let program = get_program(client);
    let tx = program
        .request()
        .accounts(klend::accounts::BorrowObligationLiquidity {
            owner: wallet.pubkey(),
            obligation,
            lending_market: Pubkey::default(),
            lending_market_authority: Pubkey::default(),
            borrow_reserve,
            reserve_source_liquidity: Pubkey::default(),
            borrow_reserve_liquidity_fee_receiver: Pubkey::default(),
            user_destination_liquidity: Pubkey::default(),
            referrer_token_state: Pubkey::default(),
            token_program: system_program::ID,
            instruction_sysvar_account: sysvar::instructions::ID,
        })
        .args(klend::instruction::BorrowObligationLiquidity {
            _liquidity_amount: amount,
        })
        .signer(wallet)
        .send();
    info!("Borrowed: {:?}", tx);
}

pub fn repay(
    client: &Client<Rc<Keypair>>,
    wallet: &Keypair,
    obligation: Pubkey,
    repay_reserve: Pubkey,
    amount: u64,
) {
    let program = get_program(client);
    let tx = program
        .request()
        .accounts(klend::accounts::RepayObligationLiquidity {
            owner: wallet.pubkey(),
            obligation,
            lending_market: Pubkey::default(),
            repay_reserve,
            reserve_destination_liquidity: Pubkey::default(),
            user_source_liquidity: Pubkey::default(),
            token_program: system_program::ID,
            instruction_sysvar_account: sysvar::instructions::ID,
        })
        .args(klend::instruction::RepayObligationLiquidity {
            _liquidity_amount: amount,
        })
        .signer(wallet)
        .send();
    info!("Repaid: {:?}", tx);
}

#[instrument(skip_all)]
fn process_rpc_error(error: ClientError) -> Error {
    let error = match error {
        ClientError::SolanaClientError(solana_rpc_client_api::client_error::Error {
            kind: error,
            ..
        }) => error,
        err => {
            tracing::error!(%err, "Anchor error");
            return Error::Misc;
        }
    };
    let (error, logs) = match error {
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
