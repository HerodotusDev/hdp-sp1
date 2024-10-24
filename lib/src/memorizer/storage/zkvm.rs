use super::StorageMemorizer;
use crate::memorizer::{
    keys::AccountKey, keys::MemorizerKey, keys::StorageKey, values::MemorizerValue, Memorizer,
    MemorizerError,
};
use crate::mpt::Mpt;
use alloy_primitives::U256;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
        let account_key: MemorizerKey = AccountKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
            address: key.address,
        }
        .into();

        if let Some(MemorizerValue::Account(account_value)) = self.map.get(&account_key) {
            let storage_root = account_value.account.storage_root;
            let storage_key: MemorizerKey = key.clone().into();

            if let Some(MemorizerValue::Storage(storage_value)) = self.map.get(&storage_key) {
                let mpt = Mpt { root: storage_root };
                println!("cycle-tracker-start: mpt(storage)");
                mpt.verify_storage(
                    storage_value.proof.clone(),
                    key.storage_slot,
                    storage_value.value.clone(),
                )?;
                println!("cycle-tracker-end: mpt(storage)");
                Ok(storage_value.value)
            } else {
                Err(MemorizerError::MissingStorage)
            }
        } else {
            println!("Missing header, {:?}", key.block_number);
            Err(MemorizerError::MissingHeader)
        }
    }
}
