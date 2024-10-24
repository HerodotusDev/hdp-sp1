use super::StorageMemorizer;
use crate::account::AccountProvider;
use crate::memorizer::values::StorageMemorizerValue;
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::StorageKey, Memorizer};
use alloy_primitives::U256;
use tokio::runtime::Runtime;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
        let rt = Runtime::new().unwrap();
        let (_, storage_proof, storage_value) = rt.block_on(async {
            let client: AccountProvider = AccountProvider::new(self.rpc_url.clone().unwrap());
            client
                .get_storage(key.address, key.block_number, key.storage_slot)
                .await
                .unwrap()
        });

        self.map.insert(
            key.into(),
            crate::memorizer::values::MemorizerValue::Storage(StorageMemorizerValue {
                value: storage_value,
                proof: storage_proof,
            }),
        );

        Ok(storage_value)
    }
}
