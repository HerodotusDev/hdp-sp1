use alloy_primitives::{Bytes, B256, U256};
use alloy_trie::{
    proof::{verify_proof, ProofVerificationError},
    Nibbles,
};

pub struct Mpt {
    pub root: B256,
}

impl Mpt {
    pub fn verify(&self, tx_index: u64, proof: Vec<Bytes>) -> Result<(), ProofVerificationError> {
        let tx_encoded = alloy_rlp::encode(U256::from(tx_index));
        let key = Bytes::from(tx_encoded);
        let nibbles = Nibbles::unpack(key);

        // TODO: last element of proof is the value of the key, not sure if it's ok to hardcode split prefix
        let expected = &proof.last().unwrap()[5..];
        verify_proof(self.root, nibbles, Some(expected.to_vec()), &proof)
    }
}
