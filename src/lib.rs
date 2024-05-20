pub mod errs;
mod eth;

use alloy::dyn_abi::abi;
use alloy::network::EthereumSigner;
use alloy::primitives::{Address, U256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::sol;
use alloy::transports::http::Http;
use std::convert::TryFrom;
use std::sync::Arc;
use tokio::runtime::Runtime;

// sol! {
//     interface UniswapV2Pair {
//         function getReserves() external view returns (uint112 reserve0, uint112 reserve1, uint32 blockTimestampLast);
//     }
// }

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Set up the Ethereum node URL, here using Infura's mainnet endpoint
//     let rpc_url = "https://mainnet.infura.io/v3/YOUR_INFURA_PROJECT_ID";
//     let http_provider = ProviderBuilder::new().on_builtin(&rpc_url).await?;

//     // Uniswap V2 ETH/USDC pair contract address
//     let pair_address: Address = "0xB4e16d0168e52d35CaCD2c6185b44281Ec28C9Dc".parse()?;

//     // The ABI for the UniswapV2Pair contract's getReserves function
//     let get_reserves_abi = "0x0902f1ac"; // This is the function selector for getReserves()

//     // Prepare the call
//     let data = get_reserves_abi.to_vec();
//     let call = provider.call(pair_address, data.into()).await?;

//     // Calculate the price (ETH/USDC)
//     let price = reserve1.as_u64() as f64 / reserve0.as_u64() as f64;
//     println!("Uniswap ETH/USDC Price: ${:.2}", price);

//     Ok(())
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
