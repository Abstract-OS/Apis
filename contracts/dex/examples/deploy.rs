use abstract_boot::ApiDeployer;

use boot_core::networks::{parse_network, NetworkInfo};
use boot_core::*;
use cosmwasm_std::Decimal;
use dex::boot::DexApi;
use dex::msg::DexInstantiateMsg;
use dex::EXCHANGE;
use semver::Version;
use std::sync::Arc;
use tokio::runtime::Runtime;

const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

fn deploy_dex(network: NetworkInfo) -> anyhow::Result<()> {
    let version: Version = CONTRACT_VERSION.parse().unwrap();
    let rt = Arc::new(Runtime::new()?);
    let options = DaemonOptionsBuilder::default().network(network).build();
    let (_sender, chain) = instantiate_daemon_env(&rt, options?)?;
    let mut dex = DexApi::new(EXCHANGE, chain);
    dex.deploy(
        version,
        DexInstantiateMsg {
            swap_fee: Decimal::percent(1),
            recipient_os: 0,
        },
    )?;
    Ok(())
}

use clap::Parser;

#[derive(Parser, Default, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Network Id to deploy on
    #[arg(short, long)]
    network_id: String,
}

fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    use dotenv::dotenv;

    let args = Arguments::parse();

    let network = parse_network(&args.network_id);

    deploy_dex(network)
}
