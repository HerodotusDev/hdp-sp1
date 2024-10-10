use alloy_primitives::B256;
use eth_trie_proofs::{tx::ConsensusTx, tx_trie::TxsMptHandler};
use url::Url;

#[derive(Debug)]
pub struct TransactionResponse {
    pub mpt_root: B256,
    pub tx: ConsensusTx,
    pub proof: Vec<Vec<u8>>,
}

pub struct TransactionClient {}

impl TransactionClient {
    pub async fn get_transaction(
        &self,
        url: Url,
        block_number: u64,
        tx_index: u64,
    ) -> Result<TransactionResponse, reqwest::Error> {
        let mut txs_mpt_handler = TxsMptHandler::new(url).unwrap();
        txs_mpt_handler
            .build_tx_tree_from_block(block_number)
            .await
            .unwrap();

        let proof = txs_mpt_handler.get_proof(tx_index).unwrap();
        let tx = txs_mpt_handler.get_tx(tx_index).unwrap();
        let tx_res = TransactionResponse {
            mpt_root: txs_mpt_handler.get_root().unwrap(),
            tx,
            proof,
        };
        Ok(tx_res)
    }
}

#[cfg(test)]
mod tests {
    use crate::mpt::Mpt;

    use super::*;

    #[tokio::test]
    async fn test_get_transaction() {
        let client = TransactionClient {};
        let url =
            Url::parse("https://mainnet.infura.io/v3/9aa3d95b3bc440fa88ea12eaa4456161").unwrap();
        let tx_res = client.get_transaction(url, 12244000, 118).await.unwrap();

        // Verify the transaction proof
        let mpt = Mpt {
            root: tx_res.mpt_root,
        };
        mpt.verify(118, tx_res.proof);
    }
}
