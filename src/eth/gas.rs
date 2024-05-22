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

use crate::{data::AddressInfo, errs::ChainXResult};

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
    let rpc_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
    let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;

    // let call = latestAnswerCall {}.abi_encode();
    // let input = Bytes::from(call);

    // let tx = TransactionRequest::default()
    //     .with_to(ETH_USD_FEED)
    //     .with_input(input);

    // let response = provider.call(&tx).block(BlockId::latest()).await?;
    // let result = U256::from_str(&response.to_string())?;

    let wei_per_gas = provider.get_gas_price().await?;
    let result = get_price_u256(provider, ETH_USD_FEED).await?;
    // Convert the gas price to Gwei and USD.
    let gwei = format_units(wei_per_gas, "gwei")?.parse::<f64>()?;
    let usd = get_usd_value(wei_per_gas, result)?;

    println!("Gas price in Gwei: {gwei}");
    println!("Gas price in USD: {usd}");

    Ok(())
}

const INFURA_PROJECT_ID: &str = "c60b0bb42f8a4c6481ecd229eddaca27";

#[inline]
async fn get_price_u256(provider: impl Provider, address: Address) -> ChainXResult<U256> {
    let call = latestAnswerCall {}.abi_encode();
    let input = Bytes::from(call);

    let tx = TransactionRequest::default()
        .with_to(address)
        .with_input(input);

    let response = provider.call(&tx).block(BlockId::latest()).await?;
    let result = U256::from_str(&response.to_string())?;

    // let usd_price = format_units(result, "usd")?.parse::<f64>()?;
    Ok(result)
}

async fn get_price_usd(provider: &impl Provider, address: Address) -> ChainXResult<f64> {
    let result = get_price_u256(provider, address).await?;
    // println!("result: {:?}", result);
    // let usd_price = format_units(result, "usd")?.parse::<f64>()?;
    let usd_price = result_to_f64_value(result, 8)?;
    Ok(usd_price)
}

// Function to convert U256 to f64 with a given number of decimal places
fn result_to_f64_value(value: U256, decimals: u32) -> ChainXResult<f64> {
    let value_str = value.to_string();
    let value_f64 = value_str.parse::<f64>()?;
    let scale = 10u64.pow(decimals) as f64;
    Ok(value_f64 / scale)
}

// async fn l2_prices() -> Result<(), Box<dyn std::error::Error>> {
//     // Create a provider
//     let rpc_url = format!("https://mainnet.infura.io/v3/{}", INFURA_PROJECT_ID);
//     let provider = ProviderBuilder::new().http(&rpc_url).await?;

//     // Get the prices for L2 solutions
//     let arbitrum_price = get_price(&provider, ARBITRUM_USD_FEED).await?;
//     let optimism_price = get_price(&provider, OPTIMISM_USD_FEED).await?;

//     println!("Arbitrum price in USD: {}", arbitrum_price);
//     println!("Optimism price in USD: {}", optimism_price);

//     Ok(())
// }

fn get_usd_value(amount: u128, price_usd: U256) -> ChainXResult<f64> {
    let base = U256::from(10).pow(U256::from(ETH_DECIMALS));
    let value = U256::from(amount) * price_usd / base;
    let formatted = format_units(value, ETH_USD_FEED_DECIMALS)?.parse::<f64>()?;

    Ok(formatted)
}

#[inline(always)]
fn get_raw_address(ai: &AddressInfo) -> ChainXResult<Address> {
    let addr = Address::from_str(&ai.address[2..])?;
    Ok(addr)
}

#[cfg(test)]
mod unit_tests {
    use super::*;
    use crate::data::get_addresses;

    #[tokio::test]
    async fn test_gas_price() -> ChainXResult<()> {
        // gas_price_usd().await?;
        let ais = get_addresses().unwrap();
        let rpc_url = "https://mainnet.infura.io/v3/c60b0bb42f8a4c6481ecd229eddaca27";
        let provider = ProviderBuilder::new().on_builtin(rpc_url).await?;
        for ai in &ais {
            let a = get_raw_address(ai)?;
            let p = get_price_usd(&provider, a).await?;
            println!("{}, {}: {}", ai.asset_name, p, ai.pair);
        }
        Ok(())
    }
}
