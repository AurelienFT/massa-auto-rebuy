mod rpc;

use std::{path::PathBuf, str::FromStr};

use massa_wallet::Wallet;
use anyhow::{Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = rpc::Client::new("51.77.221.132".parse().unwrap(), 33035).await;
    let wallet = Wallet::new(PathBuf::from("wallet.dat"))?;
    let wallet_info =  client.0.get_addresses(wallet.get_full_wallet().keys().copied().collect()).await;
    println!("{:?}", wallet_info);
    if let Ok(wallet_addresses) = wallet_info {
        if !wallet_addresses.is_empty() && wallet_addresses[0].rolls.candidate_rolls == 0 &&  wallet_addresses[0].ledger_info.final_ledger_info.balance >= massa_models::Amount::from_raw(100000000000) {
            rpc::send_operation(&client, &wallet, massa_models::OperationType::RollBuy{ roll_count: 1 }, massa_models::Amount::from_raw(0), wallet_addresses[0].address, true).await?;
        }
    }
    Ok(())
}
