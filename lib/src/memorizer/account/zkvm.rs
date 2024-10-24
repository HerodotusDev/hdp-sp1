use super::AccountMemorizer;
use crate::memorizer::{
    keys::{AccountKey, HeaderKey, MemorizerKey},
    values::MemorizerValue,
    Memorizer, MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::Account;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        let header_key: MemorizerKey = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        }
        .into();

        if let Some(MemorizerValue::Header(header_value)) = self.map.get(&header_key) {
            let state_root = header_value.header.state_root;
            let account_key: MemorizerKey = key.clone().into();

            if let Some(MemorizerValue::Account(account_value)) = self.map.get(&account_key) {
                let mpt = Mpt { root: state_root };
                println!("cycle-tracker-start: mpt(account)");
                mpt.verify_account(
                    account_value.proof.clone(),
                    account_value.account.clone(),
                    key.address,
                )?;
                println!("cycle-tracker-end: mpt(account)");
                Ok(account_value.account)
            } else {
                Err(MemorizerError::MissingAccount)
            }
        } else {
            println!("Missing header, {:?}", key.block_number);
            Err(MemorizerError::MissingHeader)
        }
    }
}
