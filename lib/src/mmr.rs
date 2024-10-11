use alloy_primitives::keccak256;
use alloy_primitives::{B256, U256};
use serde::{Deserialize, Serialize};
use thiserror_no_std::Error;

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct MmrMeta {
    pub root_hash: B256,
    pub mmr_size: u128,
    pub peaks: Vec<B256>,
}

impl MmrMeta {
    pub fn new(root_hash: B256, mmr_size: u128, peaks: Vec<B256>) -> Self {
        Self {
            root_hash,
            mmr_size,
            peaks,
        }
    }

    pub fn verify_proof(
        &self,
        element_index: u128,
        element_hash: B256,
        proof: Vec<B256>,
    ) -> Result<bool, MmrError> {
        let calculated_root = self.compute_bagged_peaks()?;

        if calculated_root != self.root_hash {
            return Err(MmrError::InvalidRootHash);
        }

        let leaf_count = mmr_size_to_leaf_count(self.mmr_size as usize);
        let expected_peak_count = leaf_count_to_peak_count(leaf_count);

        if expected_peak_count != self.peaks.len() as u32 {
            return Err(MmrError::InvalidPeakCount);
        }

        let mut current_hash = element_hash;
        let mut leaf_index = element_index_to_leaf_index(element_index as usize)?;

        for proof_element in proof {
            let is_right = leaf_index % 2 == 1;
            current_hash = if is_right {
                keccak256([proof_element, current_hash].concat())
            } else {
                keccak256([current_hash, proof_element].concat())
            };

            leaf_index /= 2;
        }

        let (peak_index, _) = get_peak_info(self.mmr_size as usize, element_index as usize);

        if self.peaks[peak_index] == current_hash {
            Ok(true)
        } else {
            Err(MmrError::InvalidProof)
        }
    }

    fn compute_bagged_peaks(&self) -> Result<B256, MmrError> {
        let final_peak = self.compute_final_peak()?;
        let size_hash: B256 = U256::from(self.mmr_size).into();
        Ok(keccak256([size_hash, final_peak].concat()))
    }

    fn compute_final_peak(&self) -> Result<B256, MmrError> {
        if self.peaks.is_empty() {
            return Err(MmrError::InvalidPeakCount);
        }

        let mut peak_hashes = self.peaks.clone();

        if peak_hashes.len() == 1 {
            return Ok(peak_hashes[0]);
        }

        let last_peak = peak_hashes.pop().unwrap();
        let second_last_peak = peak_hashes.pop().unwrap();
        let initial_root = keccak256([second_last_peak, last_peak].concat());

        Ok(peak_hashes
            .into_iter()
            .rev()
            .fold(initial_root, |prev, current| {
                keccak256([current, prev].concat())
            }))
    }

    /// P = Poseidon(N | Poseidon(N | Node(p1) | Node(p2) | Node(p3))), N = size, p = peaks
    #[cfg(test)]
    fn bag_peaks(&self) -> B256 {
        let final_top_peak = self.final_top_peak();
        let size: B256 = U256::from(self.mmr_size).into();
        keccak256([size, final_top_peak].concat())
    }

    #[cfg(test)]
    fn final_top_peak(&self) -> B256 {
        let mut peaks_hashes: Vec<B256> = self.peaks.clone();

        match peaks_hashes.len() {
            0 => panic!("Error"),
            1 => peaks_hashes[0],
            _ => {
                let last = peaks_hashes.pop().unwrap();
                let second_last = peaks_hashes.pop().unwrap();
                let root0 = keccak256([second_last, last].concat());

                peaks_hashes
                    .into_iter()
                    .rev()
                    .fold(root0, |prev, cur| keccak256([cur, prev].concat()))
            }
        }
    }
}

fn bit_length(value: usize) -> usize {
    (std::mem::size_of::<usize>() * 8) - value.leading_zeros() as usize
}

pub fn get_peak_info(mut element_count: usize, mut element_index: usize) -> (usize, usize) {
    let mut mountain_height = bit_length(element_count);
    let mut mountain_size = (1 << mountain_height) - 1;
    let mut mountain_index = 0;

    loop {
        if mountain_size <= element_count {
            if element_index <= mountain_size {
                return (mountain_index, mountain_height - 1);
            }
            element_count -= mountain_size;
            element_index -= mountain_size;
            mountain_index += 1;
        }
        mountain_size >>= 1;
        mountain_height -= 1;
    }
}

pub fn leaf_count_to_peak_count(leaf_count: usize) -> u32 {
    count_ones(leaf_count) as u32
}

pub fn count_ones(mut value: usize) -> usize {
    let mut count = 0;
    while value > 0 {
        value &= value - 1;
        count += 1;
    }
    count
}

pub fn mmr_size_to_leaf_count(mmr_size: usize) -> usize {
    let mut remaining_size = mmr_size;
    let bits = bit_length(remaining_size + 1);
    let mut tip_size = 1 << (bits - 1);
    let mut leaf_count = 0;

    while tip_size != 0 {
        let mountain_size = 2 * tip_size - 1;
        if mountain_size <= remaining_size {
            remaining_size -= mountain_size;
            leaf_count += tip_size;
        }
        tip_size >>= 1;
    }

    leaf_count
}

pub fn element_index_to_leaf_index(element_index: usize) -> Result<usize, MmrError> {
    if element_index == 0 {
        return Err(MmrError::InvalidElementIndex);
    }
    count_elements_to_leaf_count(element_index - 1)
}

pub fn count_elements_to_leaf_count(element_count: usize) -> Result<usize, MmrError> {
    let mut leaf_count = 0;
    let mut mountain_leaf_count = 1 << bit_length(element_count);
    let mut remaining_elements = element_count;

    while mountain_leaf_count > 0 {
        let mountain_size = 2 * mountain_leaf_count - 1;
        if mountain_size <= remaining_elements {
            leaf_count += mountain_leaf_count;
            remaining_elements -= mountain_size;
        }
        mountain_leaf_count >>= 1;
    }

    if remaining_elements > 0 {
        Err(MmrError::UnprocessedElements)
    } else {
        Ok(leaf_count)
    }
}

#[derive(Debug, Error)]
pub enum MmrError {
    #[error("Invalid root hash")]
    InvalidRootHash,

    #[error("Invalid proof")]
    InvalidProof,

    #[error("Invalid element index")]
    InvalidElementIndex,

    #[error("Invalid peak count")]
    InvalidPeakCount,

    #[error("Invalid size")]
    InvalidSize,

    #[error("There are unprocessed elements")]
    UnprocessedElements,

    #[error("Error decoding the data")]
    DecodingError,
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub rlp: String,
    pub proof: HeaderInclusionProof,
}

#[derive(Serialize, Deserialize)]
pub struct HeaderInclusionProof {
    pub leaf_index: u128,
    pub mmr_proof: Vec<B256>,
}

// pub fn verify_headers(mmr: &MmrMeta, headers: &[Header]) -> Result<bool, MmrError> {
//     for header in headers {
//         let rlp_bytes = hex::decode(&header.rlp).map_err(|_| MmrError::DecodingError)?;
//         let header_hash = keccak256(rlp_bytes);

//         mmr.verify_proof(
//             header.proof.leaf_index,
//             header_hash,
//             header.proof.mmr_proof.clone(),
//         )?;
//     }
//     Ok(true)
// }

// pub fn validate_mmr(mmr: &MmrMeta) -> Result<(), MmrError> {
//     if !is_valid_mmr_size(mmr.mmr_size) {
//         return Err(MmrError::InvalidSize);
//     }
//     Ok(())
// }

// fn is_valid_mmr_size(size: u128) -> bool {
//     size >= 1 && size <= 2_u128.pow(126)
// }

#[cfg(test)]
mod tests {
    use alloy_primitives::b256;
    use alloy_primitives::hex;
    use alloy_primitives::hex::FromHex;

    use super::*;

    pub fn verify_headers_with_mmr_peaks(
        mmr: MmrMeta,
        headers: &[Header],
    ) -> Result<bool, MmrError> {
        let mut is_verified = true;
        for header in headers {
            let rlp_bytes = hex::decode(&header.rlp).unwrap();
            let element_value = keccak256(rlp_bytes);
            is_verified = mmr.verify_proof(
                header.proof.leaf_index,
                element_value,
                header.proof.mmr_proof.clone(),
            )?;
        }
        Ok(is_verified)
    }

    #[test]
    fn test_bag_peaks() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root_hash: B256::from_hex(
                "0x00367542437d21fb3d94c5449b6f6e650c4b4f8f307c2d4aa3a782f17a4ddd03",
            )
            .unwrap(),
            mmr_size: 10,
            peaks: vec![
                B256::from_hex(
                    "0xb4c11951957c6f8f642c4af61cd6b24640fec6dc7fc607ee8206a99e92410d30",
                )
                .unwrap(),
                B256::from_hex(
                    "0xad3228b676f7d3cd4284a5443f17f1962b36e491b30a40b2405849e597ba5fb5",
                )
                .unwrap(),
            ],
        };

        let bag = test_mmr_meta.bag_peaks();
        assert_eq!(bag, test_mmr_meta.root_hash);
    }

    #[test]
    fn verify_proof() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root_hash: B256::from_hex(
                "0xa7122a01868e54648facd92a3a821fae03301a71d1bd02fabe4e82bffcbd0aeb",
            )
            .unwrap(),
            mmr_size: 11,
            peaks: vec![
                B256::from_hex(
                    "0xbf874bd367f32d74d7d084a8eb85ce99d6f2622fbc0d1f83dcd0c4404f8e0cea",
                )
                .unwrap(),
                B256::from_hex(
                    "0x04cde762ef08b6b6c5ded8e8c4c0b3f4e5c9ad7342c88fcc93681b4588b73f05",
                )
                .unwrap(),
                B256::from(U256::from(3)),
            ],
        };

        assert!(test_mmr_meta
            .verify_proof(
                8,
                B256::from(U256::from(5)),
                vec![B256::from(U256::from(4))],
            )
            .unwrap());
    }

    #[test]
    fn test_verify_headers_with_mmr_peaks() {
        let test_mmr_meta: MmrMeta = MmrMeta {
            root_hash: b256!("62d451ed3f131fa253957db4501b0f4b6eb3f29c706663be3f75a35b7b372a38"),
            mmr_size: 13024091,
            peaks: vec![
                b256!("ea94b197307128f1e18f9f3186a6452bd201b86f484f80cc3b2cbfb0b646c577"),
                b256!("ff430ddf60e969c483750fd56caee265cab4037f437d4a0a45eee230088e9092"),
                b256!("8735438529236334bc5b13c0bb8ba6ad62f1b0e7f821a739fcdbd7903d618d6a"),
                b256!("c86310b6895e77987c3e0afa79b0e2fac4538405a5e3ab276c915cdb4e74b4b9"),
                b256!("9dd90ca28eac4c7e903923164d9ca4e4227fb0c400ec1f9da20fa0ef33f438be"),
                b256!("73d7ed3f6cf4713925838f61e8debebbee3d33652d684488387d05712837af1e"),
                b256!("8f570e28c7fa0d9aef96bc80e1985696094fa132b47417b67429b37fb3413469"),
                b256!("5e5ad2c6f4e13950a0ddd7e0c803aa24cd968c59d104f6ac5a46631c63896273"),
                b256!("9e45d7d4fa8c5711c2df9636f3493ab31e1a12e463a0eec4798aa163d4d9a2a2"),
                b256!("481b6377529be8836be09c47917289c5218b710e2d2f186c3b96f7d404a02312"),
                b256!("f864d07f7cf26b072aa30e1223cf16f338d499fe83935836ff565c3cf9e42530"),
                b256!("6fdbe7ef87553b453ef0c66322a33575f1e92b00d2abca122f9d9caeddca03b7"),
                b256!("45da6302e5933720e03c6f851000ac3605ca863c54839c265eadc252bf7c4764"),
            ],
        };

        let test_header = Header {
            rlp: "f90264a0d0dbb039df7728af964ecc414930adaf57c762df78e7818c5e29bdaf98bc30a6a01dcc4de8dec75d7aab85b567b6ccd41ad312451b948a7413f0a142fd40d49347940000006916a87b82333f4245046623b23794c65ca0fe5710ac36eae31f8fd741ec4646295805efde7d5af87f75b6c9f3b478264c03a02351a6bd671aa027fb78d3bfb0e154fe86a39b64050eb9260fae4ae4e9f39488a04432f18e1b2ff54ce5296d462ae0586a71641906412c7e600780953edd1e8c48b901008804404e86016c08119966222d18870c08157b050006544441c05a76c6e28418045100622128069e4936248c041c089a1001130e2a26990416997904927c6491d162d2c30c2f0b08421e806a2438c885562f2b033806657b78228a48802072a3ab2400c2a6212152054c0675708adb824c8800c6511a76e40268a87d00300b64aa46c9949b614428ec20b4d7572247b012914ea7682c14fc030bbcb825c4e881620a04b7ea04ce56480682102200452c00d826a7a04a8d5a49a10036170b4096e12ed52304215a1090210d95ac1654140f600315a14a500e32059106d86162a112123280c0b0200a82062042a0842317040880f06b742256602012b3197d502c808356152c8401c9c38084013ca856846611559099d883010d0d846765746888676f312e32322e30856c696e7578a0a03574c090365f7581fd16fd2144c0de59d64c03bcbfc761ad3cb0e8c567cb438800000000000000008308e316a0c6d2ec3bda594dc497c3092ca167e4449c1b6747a076c8849bcd351add59e68e830600008405240000a07625dff7a19154e26778df000ae2e3de826d28a60f749e453e3ded6e367eeed4".to_string(),
            proof: HeaderInclusionProof {
                leaf_index: 610913,
                mmr_proof: vec![
                   b256!("d0dbb039df7728af964ecc414930adaf57c762df78e7818c5e29bdaf98bc30a6"),
                   b256!("56fd87811a4b8130b0ed91ac95df8d09d333889167ce835d655a160dda8f96a0"),
                   b256!("dcb896bddfd0cad743abde0856eb20894286ab5bd54c72c68be7577749eff562"),
                   b256!("47ffad32c9cc9b4307b5570392856c5fdc45808bbbca3dc6dda274cf7bdb2e87"),
                   b256!("4df89b9609861912e2fd4fb0156de1645b4237ea9d2f6b10cfc8da8a9a78fc33"),
                   b256!("30f39a418cc1ef1750779bc81347ce3b84e5212701cc6052310e8f3da3d426b7"),
                   b256!("8af50140398972264e6d9bd6fd0d7dc7dce4e257d09781dc29092ecd0f2d77f7"),
                   b256!("a79e43c7fd3f4f2f31753626cc9d77f99e606d59b42e24b96b4c0bd5b3b89786"),
                   b256!("be76446f1b2403b198461f7bc1eeb2d97069537bbff2a5baf49aa79902e4bdc9"),
                   b256!("4c9bb2f62bdfc7e520d6ad852d3250e642f0b2bba2abdc3f3df4546c391ff085"),
                   b256!("40dba0712aecd975f5a1b62ed1eef38bef26fd73c80c8b0f0d5583312d70696a"),
                   b256!("940929e13f84c92fd69b49f7df096a4d39f695bbfff7ffca7a571ea6b59b42c0"),
                   b256!("4ade02805cdd0c1436a62801db791a480063df1aede38e172962764b7648aa11"),
                   b256!("bc53fae7e8b5bf9288a794979315b42da843cf7d0c671607c052a50c5c8ffa56"),
                   b256!("ed90b41d5fcda611f2dd98e9aa2278234ffcb3c22aa57890b0a0fd5511eaee26"),
                   b256!("d5b49334057c35f35bb042975112506396892cc097d7726d9c3c3e7f535566a2"),
                   b256!("3a68f5d5fcff64a6cdb94d51438127565fc2a8f16257606b0edb0bba13d866fc"),
                   b256!("0beaa047b0be2e8419486b91d0427225916cf1153823fac922249776833e4a76"),
                   b256!("0091ac0829cb25940c0af1212187b796950c8c407025a03761e89404a659aba6"),
                   b256!("aa405e7324b748e592af9a8d0c6de066e33c4234e22d64d1c1ec4fe83d5ce7e5"),
                   b256!("8c67a1dac6b6ee001287b68ed17340053037fc26e42c331fba131283b61e5605"),
                   b256!("42b83addf66c9124dc11c908bb7c522347c9ee32c335bf6cb80c2140b9efb336")
                ],
            },
        };

        assert!(verify_headers_with_mmr_peaks(test_mmr_meta, &[test_header]).unwrap());
    }
}
