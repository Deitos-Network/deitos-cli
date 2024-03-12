//! `deitos-cli` is a command-line utility for interacting with the Deitos chain.
//!
//! You can see the full list of commands with `deitos-cli --help`. Most commands have
//! additional help available with `deitos-cli <command> --help`.
//!
//! ## Upload a file
//!
//! To upload a file to the chain, use the `upload` command:
//!
//! ```sh
//! deitos-cli upload --file-path <path> --deitos-url <url> --ip-url <url> --agreement <id> --suri <suri>
//! ```

use clap::Parser;

use crate::cmd::DeitosCmd;

mod chain;
mod cmd;
mod jwt;

type AgreementId = u32;
type Timestamp = u64;

#[tokio::main]
async fn main() {
    DeitosCmd::parse().run().await
}
