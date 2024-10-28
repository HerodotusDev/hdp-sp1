use super::StorageMemorizer;
use crate::account::AccountProvider;
use crate::memorizer::values::StorageMemorizerValue;
use crate::memorizer::{keys::StorageKey, Memorizer};
use crate::memorizer::{
    AccountKey, AccountMemorizerValue, HeaderKey, HeaderMemorizer, MemorizerError, MemorizerKey,
    MemorizerValue,
};
use alloy_primitives::U256;
use tokio::runtime::Runtime;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
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
        let (account, account_proof, storage_proof, storage_value) = rt.block_on(async {
            let client: AccountProvider = AccountProvider::new(rpc_url);
            client
                .get_storage(key.address, key.block_number, key.storage_slot)
                .await
                .map_err(MemorizerError::TransportError)
        })?;

        let account_key: MemorizerKey = AccountKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
            address: key.address,
        }
        .into();

        if self.map.get(&account_key).is_none() {
            self.map.insert(
                account_key,
                (
                    MemorizerValue::Account(AccountMemorizerValue {
                        account,
                        proof: account_proof,
                    }),
                    false,
                ),
            );
        }

        self.map.insert(
            key.into(),
            (
                MemorizerValue::Storage(StorageMemorizerValue {
                    value: storage_value,
                    proof: storage_proof,
                }),
                false,
            ),
        );

        Ok(storage_value)
    }
}
