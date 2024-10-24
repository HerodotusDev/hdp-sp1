use alloy_consensus::Account;
use alloy_primitives::{keccak256, Address, Bytes, B256, U256};
use alloy_rlp::encode_fixed_size;
use alloy_trie::{
    proof::{verify_proof, ProofVerificationError},
    Nibbles,
};

pub struct Mpt {
    pub root: B256,
}

impl Mpt {
    pub fn verify_transaction(
        &self,
        tx_index: u64,
        proof: Vec<Bytes>,
    ) -> Result<(), ProofVerificationError> {
        let nibbles = Nibbles::unpack(Bytes::from(alloy_rlp::encode(U256::from(tx_index))));
        // TODO: last element of proof is the value of the key, not sure if it's ok to hardcode split prefix
        let expected = &proof.last().unwrap()[5..];
        verify_proof(self.root, nibbles, Some(expected.to_vec()), &proof)
    }

    pub fn verify_receipt(
        &self,
        tx_index: u64,
        proof: Vec<Bytes>,
    ) -> Result<(), ProofVerificationError> {
        let nibbles = Nibbles::unpack(Bytes::from(alloy_rlp::encode(U256::from(tx_index))));
        // TODO: last element of proof is the value of the key, not sure if it's ok to hardcode split prefix
        let expected = &proof.last().unwrap()[7..];
        verify_proof(self.root, nibbles, Some(expected.to_vec()), &proof)
    }

    pub fn verify_account(
        &self,
        proof: Vec<Bytes>,
        account: Account,
        address: Address,
    ) -> Result<(), ProofVerificationError> {
        let nibbles = Nibbles::unpack(keccak256(address));
        let expected = alloy_rlp::encode(account);
        verify_proof(self.root, nibbles, Some(expected), &proof)
    }

    pub fn verify_storage(
        &self,
        proof: Vec<Bytes>,
        key: B256,
        value: U256,
    ) -> Result<(), ProofVerificationError> {
        let nibbles = Nibbles::unpack(keccak256(key));
        let expected = if value.is_zero() {
            None
        } else {
            Some(encode_fixed_size(&value).to_vec())
        };
        verify_proof(self.root, nibbles, expected, &proof)
    }
}
