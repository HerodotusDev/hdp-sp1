use alloy_primitives::{Bytes, B256};
use eth_trie_proofs::{
    tx::ConsensusTx, tx_receipt::ConsensusTxReceipt, tx_receipt_trie::TxReceiptsMptHandler,
    tx_trie::TxsMptHandler, EthTrieError,
};
use url::Url;

#[derive(Debug)]
pub struct TransactionResponse {
    pub tx_index: u64,
    pub mpt_root: B256,
    pub tx: ConsensusTx,
    pub proof: Vec<Bytes>,
}

#[derive(Debug)]
pub struct ReceiptResponse {
    pub tx_index: u64,
    pub mpt_root: B256,
    pub receipt: ConsensusTxReceipt,
    pub proof: Vec<Bytes>,
}

pub struct TransactionClient {}

impl Default for TransactionClient {
    fn default() -> Self {
        Self::new()
    }
}

impl TransactionClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_transaction(
        &self,
        url: Url,
        block_number: u64,
        tx_index: u64,
    ) -> Result<TransactionResponse, EthTrieError> {
        let mut txs_mpt_handler = TxsMptHandler::new(url)?;
        txs_mpt_handler
            .build_tx_tree_from_block(block_number)
            .await?;
        let proof = txs_mpt_handler.get_proof(tx_index)?;
        let proof: Vec<Bytes> = proof.into_iter().map(Bytes::from).collect();
        let tx = txs_mpt_handler.get_tx(tx_index)?;
        let tx_res = TransactionResponse {
            tx_index,
            mpt_root: txs_mpt_handler.get_root()?,
            tx,
            proof,
        };
        Ok(tx_res)
    }

    pub async fn get_receipt(
        &self,
        url: Url,
        block_number: u64,
        tx_index: u64,
    ) -> Result<ReceiptResponse, EthTrieError> {
        let mut receipt_mpt_handler = TxReceiptsMptHandler::new(url)?;
        receipt_mpt_handler
            .build_tx_receipts_tree_from_block(block_number)
            .await?;
        let proof = receipt_mpt_handler.get_proof(tx_index)?;
        let proof: Vec<Bytes> = proof.into_iter().map(Bytes::from).collect();
        let receipt = receipt_mpt_handler.get_tx_receipt(tx_index)?;
        let tx_res = ReceiptResponse {
            tx_index,
            mpt_root: receipt_mpt_handler.get_root()?,
            receipt,
            proof,
        };
        Ok(tx_res)
    }
}

#[cfg(test)]
mod tests {
    use crate::{chain::ChainId, mpt::Mpt, utils::get_rpc_urls};

    use super::*;

    #[tokio::test]
    async fn test_get_transaction() {
        let client = TransactionClient {};
        let chain_map = get_rpc_urls();
        let url = chain_map.get(&ChainId::EthereumSepolia).unwrap();
        let tx_res = client
            .get_transaction(url.clone(), 5244634, 2)
            .await
            .unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: tx_res.mpt_root,
        };
        mpt.verify_transaction(2, tx_res.proof).unwrap();
    }

    #[tokio::test]
    async fn test_get_receipt() {
        let client = TransactionClient {};
        let chain_map = get_rpc_urls();
        let url = chain_map.get(&ChainId::EthereumSepolia).unwrap().to_owned();
        let tx_res = client.get_receipt(url, 5244634, 2).await.unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: tx_res.mpt_root,
        };
        mpt.verify_receipt(2, tx_res.proof).unwrap();
    }
}
