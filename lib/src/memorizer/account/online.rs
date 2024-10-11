use super::AccountMemorizer;
use crate::memorizer::values::{AccountMemorizerValue, MemorizerValue};
use crate::memorizer::MemorizerError;
use crate::memorizer::{keys::AccountKey, Memorizer};
use alloy_consensus::Account;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Bytes, B256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_rpc_types::EIP1186AccountProofResponse;
use tokio::runtime::Runtime;

impl AccountMemorizer for Memorizer {
    fn get_account(&mut self, key: AccountKey) -> Result<Account, MemorizerError> {
        let rt = Runtime::new().unwrap();
        let (account, proof): (Account, Vec<Bytes>) = rt.block_on(async {
            let client: ReqwestClient =
                ClientBuilder::default().http(self.rpc_url.clone().unwrap());
            let mut batch = client.new_batch();

            let block_header_fut: alloy_rpc_client::Waiter<EIP1186AccountProofResponse> = batch
                .add_call(
                    "eth_getProof",
                    &(
                        key.address,
                        Vec::<B256>::new(),
                        BlockNumberOrTag::from(key.block_number),
                    ),
                )
                .unwrap();

            batch.send().await.unwrap();
            let response: EIP1186AccountProofResponse = block_header_fut.await.unwrap();
            let convert: Account = Account {
                nonce: response.nonce,
                balance: response.balance,
                code_hash: response.code_hash,
                storage_root: response.storage_hash,
            };

            (convert, response.account_proof)
        });

        self.map.insert(
            key.into(),
            MemorizerValue::Account(AccountMemorizerValue { account, proof }),
        );

        Ok(account)
    }
}
