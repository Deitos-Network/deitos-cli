//! # Chain
//!
//! This module contains the logic for interacting with the Deitos chain.
//! It uses the `subxt` crate to interact with the chain.

use sp_core::sr25519::Pair;
use subxt::{
    storage::address::Yes, storage::StorageAddress, tx::PairSigner, OnlineClient, SubstrateConfig,
};

use crate::{
    chain::deitos::runtime_types::bounded_collections::bounded_vec::BoundedVec, AgreementId,
    Timestamp,
};

#[subxt::subxt(runtime_metadata_path = "deitos.scale")]
pub mod deitos {}

/// A client for interacting with the Deitos chain.
pub struct Client {
    pub client: OnlineClient<SubstrateConfig>,
}

impl Client {
    /// Create a new client for interacting with the Deitos chain.
    pub async fn new(url: impl AsRef<str>) -> Self {
        let client = OnlineClient::from_insecure_url(url)
            .await
            .expect("Deitos node should be available");
        Self { client }
    }

    /// Get the current timestamp from the chain.
    pub async fn get_timestamp(&self) -> Option<Timestamp> {
        let query = deitos::storage().timestamp().now();
        self.fetch_or_die(query).await
    }

    pub async fn register_file(
        &self,
        signer: Pair,
        agreement_id: AgreementId,
        file_hash: String,
        file_name: String,
    ) {
        let file_hash = file_hash
            .into_bytes()
            .try_into()
            .expect("File hash should be 64 bytes");
        let file_name = BoundedVec(file_name.into_bytes());
        let tx = deitos::tx()
            .deitos_fs()
            .register_file(agreement_id, file_hash, file_name);

        let signer = PairSigner::<SubstrateConfig, _>::new(signer);
        let tx_progress = self
            .client
            .tx()
            .sign_and_submit_then_watch_default(&tx, &signer)
            .await
            .expect("File registration should submit successfully");
        println!("File registration submitted, waiting for transaction to be finalized...");

        let _events = tx_progress
            .wait_for_finalized_success()
            .await
            .expect("File registration should finalize");
        println!("File registered successfully");
    }

    async fn fetch_or_die<'a, K, V>(&self, query: K) -> Option<V>
    where
        K: StorageAddress<IsFetchable = Yes, Target = V> + 'a,
    {
        self.client
            .storage()
            .at_latest()
            .await
            .expect("Latest block should be available")
            .fetch(&query)
            .await
            .expect("Data should fetch successfully")
    }
}
