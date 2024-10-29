use alloy_consensus::Account;
use alloy_eips::BlockNumberOrTag;
use alloy_primitives::{Address, Bytes, B256, U256};
use alloy_rpc_client::{ClientBuilder, ReqwestClient};
use alloy_rpc_types::EIP1186AccountProofResponse;
use url::Url;

/// A provider for accessing account and storage proofs
#[derive(Debug)]
pub struct AccountProvider {
    /// The RPC client.
    pub client: ReqwestClient,
}

impl AccountProvider {
    /// Creates a new [`AccountProvider`] instance with the given RPC URL
    pub fn new(rpc_url: Url) -> Self {
        Self {
            client: ClientBuilder::default().http(rpc_url.clone()),
        }
    }

    /// Fetches the account data and account proof for a given Ethereum address at a specified block.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alloy_primitives::Address;
    /// use hdp_lib::*;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let provider = AccountProvider::new(Url::parse("https://YOUR_RPC_URL").unwrap());
    ///     let address = Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap();
    ///     let block_number = 5641516;
    ///
    ///     match provider.get_account(address, block_number).await {
    ///         Ok((account, proof)) => {
    ///             println!("Account: {:?}", account);
    ///             println!("Proof: {:?}", proof);
    ///         }
    ///         Err(e) => eprintln!("Error fetching account: {:?}", e),
    ///     }
    /// }
    /// ```
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

    /// Fetches storage data for a given Ethereum address and storage slot at a specific block.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use alloy_primitives::{Address, B256, U256};
    /// use hdp_lib::AccountProvider;
    /// use std::str::FromStr;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     let provider = AccountProvider::new(Url::parse("https://YOUR_RPC_URL").unwrap());
    ///     let address = Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap();
    ///     let block_number = 5641516;
    ///     let storage_slot: B256 = U256::from(1).into();
    ///
    ///     match provider.get_storage(address, block_number, storage_slot).await {
    ///         Ok((account, account_proof, storage_proof, storage_value)) => {
    ///             println!("Account: {:?}", account);
    ///             println!("Account Proof: {:?}", account_proof);
    ///             println!("Storage Proof: {:?}", storage_proof);
    ///             println!("Storage Value: {:?}", storage_value);
    ///         }
    ///         Err(e) => eprintln!("Error fetching storage: {:?}", e),
    ///     }
    /// }
    /// ```
    pub async fn get_storage(
        &self,
        address: Address,
        block_number: u64,
        storage_slot: B256,
    ) -> Result<(Account, Vec<Bytes>, Vec<Bytes>, U256), alloy_transport::TransportError> {
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
        let converted_account: Account = Account {
            nonce: response.nonce,
            balance: response.balance,
            code_hash: response.code_hash,
            storage_root: response.storage_hash,
        };
        let storage_proof = response.storage_proof[0].proof.clone();
        let storage_value = response.storage_proof[0].value;
        Ok((
            converted_account,
            response.account_proof,
            storage_proof,
            storage_value,
        ))
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
        let (account, _account_proof, storage_proof, storage_value) = provider
            .get_storage(
                Address::from_str("0x75cec1db9dceb703200eaa6595f66885c962b920").unwrap(),
                5641516,
                storage_key,
            )
            .await
            .unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: account.storage_root,
        };
        mpt.verify_storage(storage_proof, storage_key, storage_value)
            .unwrap();
    }
}
