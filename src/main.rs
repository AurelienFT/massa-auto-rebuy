mod rpc;

use std::{path::PathBuf};

use massa_wallet::Wallet;
use anyhow::{Result};

#[paw::main]
#[tokio::main]
async fn main(args: paw::Args) -> Result<()> {
    let mut args = args.skip(1);

    let ip = args
        .next()
        .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "the host argument is missing"))?;
    let port = args
        .next()
        .unwrap_or("33035".to_string()).parse().unwrap();
    let client = rpc::Client::new(ip.parse().unwrap(), port).await;
    let wallet = Wallet::new(PathBuf::from("wallet.dat"))?;
    let wallet_info =  client.0.get_addresses(wallet.get_full_wallet().keys().copied().collect()).await;
    if let Ok(wallet_addresses) = wallet_info {
        if !wallet_addresses.is_empty() && wallet_addresses[0].rolls.candidate_rolls == 0 &&  wallet_addresses[0].ledger_info.final_ledger_info.balance >= massa_models::Amount::from_raw(100000000000) {
            rpc::send_operation(&client, &wallet, massa_models::OperationType::RollBuy{ roll_count: 1 }, massa_models::Amount::from_raw(0), wallet_addresses[0].address, true).await?;
        }
    }
    Ok(())
}
