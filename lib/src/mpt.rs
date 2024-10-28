use alloy_consensus::Account;
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_rlp::encode_fixed_size;
use alloy_trie::{
    proof::{verify_proof, ProofVerificationError},
    Nibbles,
};
use thiserror_no_std::Error;

/// Represents a Merkle Patricia Tree (MPT).
/// to verify transactions, receipts, accounts, and storage proofs.
#[derive(Debug)]
pub struct Mpt {
    /// The root hash of the MPT, representing the authenticated state.
    pub root: B256,
}

impl Mpt {
    /// Creates a new `Mpt` instance with a specified root hash.
    ///
    /// # Arguments
    /// * `root` - The root hash of the MPT.
    pub fn new(root: B256) -> Self {
        Self { root }
    }

    /// Verifies a transaction in the MPT using a proof.
    ///
    /// # Arguments
    /// * `tx_index` - The index of the transaction.
    /// * `proof` - The proof elements required to verify the transaction.
    ///
    /// # Returns
    /// A `Result` which is `Ok(())` if the proof is valid, or an [`MptError`] otherwise.
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

    /// Verifies a receipt in the MPT using a proof.
    ///
    /// # Arguments
    /// * `tx_index` - The index of the transaction's receipt.
    /// * `proof` - The proof elements required to verify the receipt.
    ///
    /// # Returns
    /// A `Result` which is `Ok(())` if the proof is valid, or an [`MptError`] otherwise.
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

    /// Verifies an account in the MPT using a proof.
    ///
    /// # Arguments
    /// * `proof` - The proof elements required to verify the account.
    /// * `account` - The account data to verify.
    /// * `address` - The address of the account.
    ///
    /// # Returns
    /// A `Result` which is `Ok(())` if the proof is valid, or an [`MptError`] otherwise.
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

    /// Verifies a storage value in the MPT using a proof.
    ///
    /// # Arguments
    /// * `proof` - The proof elements required to verify the storage.
    /// * `key` - The storage key.
    /// * `value` - The expected storage value.
    ///
    /// # Returns
    /// A `Result` which is `Ok(())` if the proof is valid, or an [`MptError`] otherwise.
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

/// Error types that may occur during MPT operations.
#[derive(Debug, Error)]
pub enum MptError {
    /// Error when the proof verification fails.
    #[error(transparent)]
    ProofVerification(#[from] ProofVerificationError),

    /// Error when the proof provided is invalid or incomplete.
    #[error("Invalid proof")]
    InvalidProof,
}
