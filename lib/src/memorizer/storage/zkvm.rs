use super::StorageMemorizer;
use crate::memorizer::{
    keys::{AccountKey, HeaderKey, MemorizerKey, StorageKey},
    values::MemorizerValue,
    AccountMemorizer, HeaderMemorizer, Memorizer, MemorizerError,
};
use crate::mpt::Mpt;
use alloy_primitives::U256;

impl StorageMemorizer for Memorizer {
    fn get_storage(&mut self, key: StorageKey) -> Result<U256, MemorizerError> {
        // 1. Header
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let header = self.get_header(header_key)?;

        // 2. Account
        let account_key = AccountKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
            address: key.address,
        };
        let account = self.get_account(account_key)?;

        // 3. Storage
        let storage_root = account.storage_root;
        let storage_key: MemorizerKey = key.clone().into();

        if let Some((MemorizerValue::Storage(storage_value), is_verified)) =
            self.map.get_mut(&storage_key)
        {
            if *is_verified {
                println!("Storage MPT already verified");
                Ok(storage_value.value)
            } else {
                let mpt = Mpt { root: storage_root };
                println!("cycle-tracker-start: mpt(storage)");
                mpt.verify_storage(
                    storage_value.proof.clone(),
                    key.storage_slot,
                    storage_value.value.clone(),
                )?;
                println!("cycle-tracker-end: mpt(storage)");
                *is_verified = true;
                Ok(storage_value.value)
            }
        } else {
            Err(MemorizerError::MissingStorage)
        }
    }
}
