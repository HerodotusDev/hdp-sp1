use alloy_consensus::Account;
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_rlp::encode_fixed_size;
use alloy_trie::{
    proof::{verify_proof, ProofVerificationError},
    Nibbles,
};
use thiserror_no_std::Error;

#[derive(Debug)]
pub struct Mpt {
    pub root: B256,
}

impl Mpt {
    pub fn new(root: B256) -> Self {
        Self { root }
    }

    pub fn verify_transaction(&self, tx_index: u64, proof: Vec<Bytes>) -> Result<(), MptError> {
        let nibbles = Nibbles::unpack(Bytes::from(alloy_rlp::encode(U256::from(tx_index))));
        // TODO: last element of proof is the value of the key, not sure if it's ok to hardcode split prefix
        let expected_value = proof
            .last()
            .and_then(|bytes| bytes.get(5..))
            .ok_or(MptError::InvalidProof)?;
        verify_proof(self.root, nibbles, Some(expected_value.to_vec()), &proof)
            .map_err(MptError::ProofVerification)
    }

    pub fn verify_receipt(&self, tx_index: u64, proof: Vec<Bytes>) -> Result<(), MptError> {
        let nibbles = Nibbles::unpack(Bytes::from(alloy_rlp::encode(U256::from(tx_index))));
        // TODO: last element of proof is the value of the key, not sure if it's ok to hardcode split prefix
        let expected_value = proof
            .last()
            .and_then(|bytes| bytes.get(7..))
            .ok_or(MptError::InvalidProof)?;
        verify_proof(self.root, nibbles, Some(expected_value.to_vec()), &proof)
            .map_err(MptError::ProofVerification)
    }

    pub fn verify_account(
        &self,
        proof: Vec<Bytes>,
        account: Account,
        address: Address,
    ) -> Result<(), MptError> {
        let nibbles = Nibbles::unpack(keccak256(address));
        let expected = alloy_rlp::encode(account);
        verify_proof(self.root, nibbles, Some(expected), &proof)
            .map_err(MptError::ProofVerification)
    }

    pub fn verify_storage(
        &self,
        proof: Vec<Bytes>,
        key: B256,
        value: U256,
    ) -> Result<(), MptError> {
        let nibbles = Nibbles::unpack(keccak256(key));
        let expected = if value.is_zero() {
            None
        } else {
            Some(encode_fixed_size(&value).to_vec())
        };
        verify_proof(self.root, nibbles, expected, &proof).map_err(MptError::ProofVerification)
    }
}

#[derive(Debug, Error)]
pub enum MptError {
    #[error(transparent)]
    ProofVerification(#[from] ProofVerificationError),

    #[error("Invalid proof")]
    InvalidProof,
}
