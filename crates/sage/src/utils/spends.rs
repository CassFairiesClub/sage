use chia::{
    bls::Signature,
    protocol::{CoinSpend, SpendBundle},
};
use chia_wallet_sdk::signer::AggSigConstants;
use sage_wallet::{insert_transaction, SyncCommand, Transaction};

use crate::{Error, Result, Sage};

impl Sage {
    pub(crate) async fn sign(
        &self,
        coin_spends: Vec<CoinSpend>,
        partial: bool,
    ) -> Result<SpendBundle> {
        let wallet = self.wallet()?;

        let (_mnemonic, Some(master_sk)) =
            self.keychain.extract_secrets(wallet.fingerprint, b"")?
        else {
            return Err(Error::NoSigningKey);
        };

        let spend_bundle = wallet
            .sign_transaction(
                SpendBundle::new(coin_spends, Signature::default()),
                &AggSigConstants::new(self.network().agg_sig_me()),
                master_sk,
                partial,
            )
            .await?;

        Ok(spend_bundle)
    }

    pub(crate) async fn submit(&self, spend_bundle: SpendBundle) -> Result<()> {
        let wallet = self.wallet()?;
        let peer = self
            .state
            .peers
            .lock()
            .await
            .acquire_peer()
            .ok_or(Error::NoPeers)?;

        let subscriptions = insert_transaction(
            &wallet.db,
            &peer,
            wallet.genesis_challenge,
            spend_bundle.name(),
            Transaction::from_coin_spends(spend_bundle.coin_spends)?,
            spend_bundle.aggregated_signature,
        )
        .await?;

        self.state
            .commands
            .send(SyncCommand::SubscribeCoins {
                coin_ids: subscriptions,
            })
            .await?;

        Ok(())
    }
}
