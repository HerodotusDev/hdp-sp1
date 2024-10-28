use super::AccountMemorizer;
use crate::memorizer::{
    keys::{AccountKey, HeaderKey, MemorizerKey},
    values::MemorizerValue,
    HeaderMemorizer, Memorizer, MemorizerError,
};
use crate::mpt::Mpt;
use alloy_consensus::Account;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        // 1. Header
        let header_key = HeaderKey {
            block_number: key.block_number,
            chain_id: key.chain_id,
        };
        let header = self.get_header(header_key)?;

        // 2. Account
        let state_root = header.state_root;
        let account_key: MemorizerKey = key.clone().into();

        if let Some((MemorizerValue::Account(account_value), is_verified)) =
            self.map.get_mut(&account_key)
        {
            if *is_verified {
                println!("Account MPT already verified");
                Ok(account_value.account)
            } else {
                let mpt = Mpt { root: state_root };
                println!("cycle-tracker-start: mpt(account)");
                mpt.verify_account(
                    account_value.proof.clone(),
                    account_value.account.clone(),
                    key.address,
                )?;
                println!("cycle-tracker-end: mpt(account)");
                *is_verified = true;
                Ok(account_value.account)
            }
        } else {
            Err(MemorizerError::MissingAccount)
        }
    }
}
