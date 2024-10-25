use super::AccountMemorizer;
use crate::account::AccountProvider;
use crate::memorizer::values::{AccountMemorizerValue, MemorizerValue};
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::AccountKey, Memorizer};
use alloy_consensus::Account;
use alloy_primitives::Bytes;
use tokio::runtime::Runtime;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        let rt = Runtime::new()?;
        let rpc_url = self
            .chain_map
            .get(&key.chain_id)
            .ok_or(MemorizerError::MissingRpcUrl(key.chain_id))?
            .to_owned();
        let (account, proof): (Account, Vec<Bytes>) = rt.block_on(async {
            let client: AccountProvider = AccountProvider::new(rpc_url);
            client
                .get_account(key.address, key.block_number)
                .await
                .map_err(MemorizerError::TransportError)
        })?;

        self.map.insert(
            key.into(),
            MemorizerValue::Account(AccountMemorizerValue { account, proof }),
        );

        Ok(account)
    }
}
