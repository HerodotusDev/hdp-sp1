use std::error::Error;

use alloy_consensus::Account;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, Bytes, B256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_rpc_types::EIP1186AccountProofResponse;
use url::Url;

pub struct AccountProvider {
    pub client: ReqwestClient,
}

impl AccountProvider {
    pub fn new(rpc_url: Url) -> Self {
        Self {
            client: ClientBuilder::default().http(rpc_url.clone()),
        }
    }

    pub async fn get_account(
        &self,
        address: Address,
        block_number: u64,
    ) -> Result<(Account, Vec<Bytes>), Box<dyn Error>> {
        let mut batch = self.client.new_batch();
        let block_header_fut: alloy_rpc_client::Waiter<EIP1186AccountProofResponse> = batch
            .add_call(
                "eth_getProof",
                &(
                    address,
                    Vec::<B256>::new(),
                    BlockNumberOrTag::from(block_number),
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
        Ok((convert, response.account_proof))
    }
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use alloy_consensus::Header;

    use crate::{header::IndexerClient, mpt::Mpt};

    use super::*;

    #[tokio::test]
    async fn test_get_account() {
        let url = Url::parse("https://sepolia.ethereum.iosis.tech").unwrap();

        let client = IndexerClient::default();
        let indexer_rpc = client.get_header(5641516).await.unwrap();
        let header: Header = indexer_rpc
            .proofs
            .first()
            .unwrap()
            .rlp_block_header
            .clone()
            .into();

        let provider = AccountProvider::new(url);
        let (account, proof) = provider
            .get_account(
                Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap(),
                5641516,
            )
            .await
            .unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: header.state_root,
        };
        mpt.verify_account(
            proof,
            account,
            Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap(),
        )
        .unwrap();
    }
}
