use super::AccountMemorizer;
use crate::account::AccountProvider;
use crate::memorizer::values::{AccountMemorizerValue, MemorizerValue};
use crate::memorizer::{keys::AccountKey, Memorizer};
use crate::memorizer::{HeaderKey, HeaderMemorizer, MemorizerError};
use alloy_consensus::Account;
use alloy_primitives::Bytes;
use tokio::runtime::Runtime;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let _ = self.get_header(header_key)?;

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
            (
                MemorizerValue::Account(AccountMemorizerValue { account, proof }),
                false,
            ),
        );

        Ok(account)
    }
}
