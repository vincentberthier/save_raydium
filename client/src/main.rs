#![feature(assert_matches)]

mod config;
mod error;
mod klend;
mod lending;
mod transaction;

use std::rc::Rc;

use anchor_client::{Client, Cluster};
use clap::{Parser, Subcommand};
use config::{BSOL_MINT, RPC_HTTP, RPC_WS, TRX_PAYER, WSOL_MINT};
use klend::{get_program, init_lending_market};
use solana_sdk::pubkey;
use solana_sdk::signature::read_keypair_file;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _};

type Error = Box<dyn core::error::Error>;
type Result<T> = core::result::Result<T, Error>;

const WSOL_SOURCE: Pubkey = pubkey!("CzHgrJsCNMayNCfxLZiyghyasDw3TkDGhJKDHZDQr8qd");
const BSOL_SOURCE: Pubkey = pubkey!("FtyYfaF1w7qZVHjLwB9mb4mhSjiFh1Fc1dWbQyrhN6dT");

#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    admin: String,
    #[arg(short, long)]
    user: String,

    #[arg(short, long, default_value_t = String::from("https://api.devnet.solana.com"))]
    rpc: String,

    #[arg(short, long, default_value_t = String::from("wss://api.devnet.solana.com/"))]
    ws: String,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Test,
}

fn main() -> Result<()> {
    setup_tracing()?;
    info!("Hello World");
    let cli = Cli::parse();

    let admin = Rc::new(read_keypair_file(&cli.admin)?);
    let client = Client::new(Cluster::Devnet, Rc::clone(&admin));

    setup(&cli, &admin);

    let res = match &cli.command {
        Some(Commands::Test) => run_test(&cli, &client, &admin),
        Some(Commands::Init) => run_init(&client, &admin),
        None => {
            error!("at least one command must be given (init or test)");
            return Err("missing command".into());
        }
    };

    match res {
        Ok(()) => (),
        Err(err) => error!("failed to complete: {err}"),
    }
    Ok(())
}

#[expect(clippy::unwrap_used)]
fn setup(cli: &Cli, admin: &Keypair) {
    RPC_HTTP.set(cli.rpc.clone()).unwrap();
    RPC_WS.set(cli.ws.clone()).unwrap();
    TRX_PAYER.set(admin.to_bytes()).unwrap();
}

fn run_test(cli: &Cli, _client: &Client<Rc<Keypair>>, admin: &Keypair) -> Result<()> {
    info!("running test");

    let user = read_keypair_file(&cli.user)?;

    debug!("Admin key: {}", admin.pubkey());
    debug!("User key: {}", user.pubkey());

    Ok(())
}

fn run_init(client: &Client<Rc<Keypair>>, admin: &Keypair) -> Result<()> {
    info!("Initializing tests");

    let market = Keypair::new();
    info!("Market address: {}", market.pubkey());

    init_lending_market(client, admin, &market)?;

    Ok(())
}

fn setup_tracing() -> Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env()
        .map_err(|err| format!("could not configure tracing: {err}"))?;

    // register layers
    tracing_subscriber::registry()
        .with(filter)
        .with(fmt::layer())
        .init();
    Ok(())
}
