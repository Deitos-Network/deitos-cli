use clap::Parser;

use crate::cmd::upload::UploadCmd;

mod upload;

#[derive(Debug, Parser)]
#[command(
    name = "deitos",
    about = "Utility for file manipulation on Deitos chain",
    version
)]
pub enum DeitosCmd {
    /// Upload a file to the chain, with a given (secret) key
    Upload(UploadCmd),
}

impl DeitosCmd {
    pub async fn run(&self) {
        match self {
            DeitosCmd::Upload(cmd) => cmd.run().await,
        }
    }
}
