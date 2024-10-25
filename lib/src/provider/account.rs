use alloy_consensus::Account;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, Bytes, B256, U256};
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
    ) -> Result<(Account, Vec<Bytes>), alloy_transport::TransportError> {
        let mut batch = self.client.new_batch();
        let block_header_fut: alloy_rpc_client::Waiter<EIP1186AccountProofResponse> = batch
            .add_call(
                "eth_getProof",
                &(
                    address,
                    Vec::<B256>::new(),
                    BlockNumberOrTag::from(block_number),
                ),
            )?;
        batch.send().await?;
        let response: EIP1186AccountProofResponse = block_header_fut.await?;
        let convert: Account = Account {
            nonce: response.nonce,
            balance: response.balance,
            code_hash: response.code_hash,
            storage_root: response.storage_hash,
        };
        Ok((convert, response.account_proof))
    }

    pub async fn get_storage(
        &self,
        address: Address,
        block_number: u64,
        storage_slot: B256,
    ) -> Result<(B256, Vec<Bytes>, U256), alloy_transport::TransportError> {
        let mut batch = self.client.new_batch();
        let block_header_fut: alloy_rpc_client::Waiter<EIP1186AccountProofResponse> = batch
            .add_call(
                "eth_getProof",
                &(
                    address,
                    vec![storage_slot],
                    BlockNumberOrTag::from(block_number),
                ),
            )?;
        batch.send().await?;
        let response: EIP1186AccountProofResponse = block_header_fut.await?;
        let storage_proof = response.storage_proof[0].proof.clone();
        let storage_value = response.storage_proof[0].value;
        Ok((response.storage_hash, storage_proof, storage_value))
    }
}

#[cfg(test)]
mod tests {
    use crate::{chain::ChainId, header::IndexerClient, mpt::Mpt, utils::get_rpc_urls};
    use alloy_consensus::Header;
    use alloy_primitives::U256;
    use std::str::FromStr;

    use super::*;

    #[tokio::test]
    async fn test_get_account() {
        let chain_map = get_rpc_urls();
        let url = chain_map.get(&ChainId::EthereumSepolia).unwrap().to_owned();
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
        let target_account =
            Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap();
        let (account, proof) = provider.get_account(target_account, 5641516).await.unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: header.state_root,
        };
        mpt.verify_account(proof, account, target_account).unwrap();
    }

    #[tokio::test]
    async fn test_get_storage() {
        let chain_map = get_rpc_urls();
        let url = chain_map.get(&ChainId::EthereumSepolia).unwrap().to_owned();
        let provider = AccountProvider::new(url);
        let storage_key: B256 = U256::from(1).into();
        let (storage_root, storage_proof, storage_value) = provider
            .get_storage(
                Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap(),
                5641516,
                storage_key,
            )
            .await
            .unwrap();

        // Verify the transaction proof
        let mpt = Mpt { root: storage_root };
        mpt.verify_storage(storage_proof, storage_key, storage_value)
            .unwrap();
    }
}
