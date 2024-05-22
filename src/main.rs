use chainx::eth::prices::pretty_print_prices;
use clap::{Arg, Args, Command, Parser, Subcommand};

/**
 * features:
 * 1. clap based command line interface
 * 2. support command: price.
 *    + 2.1 the command `price`, will print the prices by call pretty_print function
*/

#[derive(Parser)]
#[clap(name = "ChainX")]
#[clap(author = "MJZK")]
#[clap(version = "1.0.0")]
#[clap(about = "ChainX is a decentralized copilot for DeFi data.", long_about = None)]
#[clap(propagate_version = true)]
pub struct ChainXCmd {
    #[clap(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand, Debug)]
enum Cmd {
    /// Prints the prices of supported chain assets.
    Price(Price),
}

#[derive(Args, Debug)]
pub struct Price {}

fn main() {
    let cmd = ChainXCmd::parse();
    match cmd.cmd {
        Cmd::Price(_price) => {
            tokio::runtime::Runtime::new().unwrap().block_on(async {
                let _ = pretty_print_prices().await;
            });
        }
    }
}
