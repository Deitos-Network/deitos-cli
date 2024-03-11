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
