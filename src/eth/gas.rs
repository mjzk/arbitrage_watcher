//! Example of how to get the gas price in USD using the Chainlink ETH/USD feed.

use alloy::{
    network::TransactionBuilder,
    primitives::{address, utils::format_units, Address, Bytes, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::eth::{BlockId, TransactionRequest},
    sol,
    sol_types::SolCall,
};
use std::str::FromStr;

use crate::errs::ChainXResult;

const ETH_USD_FEED: Address = address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419");
const ETH_USD_FEED_DECIMALS: u8 = 8;
const ETH_DECIMALS: u32 = 18;

// Codegen from excerpt of Chainlink Aggregator interface.
// See: https://etherscan.io/address/0x5f4eC3Df9cbd43714FE2740f5E3616155c5b8419#code
sol!(
    #[allow(missing_docs)]
    function latestAnswer() external view returns (int256);
);

async fn gas_price_usd() -> ChainXResult<()> {
    // Create a provider.
    let rpc_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
    let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;

    // Create a call to get the latest answer from the Chainlink ETH/USD feed.
    let call = latestAnswerCall {}.abi_encode();
    let input = Bytes::from(call);

    // Call the Chainlink ETH/USD feed contract.
    let tx = TransactionRequest::default()
        .with_to(ETH_USD_FEED)
        .with_input(input);

    let response = provider.call(&tx).block(BlockId::latest()).await?;
    let result = U256::from_str(&response.to_string())?;

    // Get the gas price of the network.
    let wei_per_gas = provider.get_gas_price().await?;

    // Convert the gas price to Gwei and USD.
    let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
    let usd = get_usd_value(wei_per_gas, result)?;

    println!("Gas price in Gwei: {gwei}");
    println!("Gas price in USD: {usd}");

    Ok(())
}

fn get_usd_value(amount: u128, price_usd: U256) -> ChainXResult<f64> {
    let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value = U256::from(amount) * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

    Ok(formatted)
}

#[cfg(test)]
mod unit_tests {
    use super::*;

    #[tokio::test]
    async fn test_gas_price() -> ChainXResult<()> {
        gas_price_usd().await?;
        // println!("{}", p);
        Ok(())
    }
}
