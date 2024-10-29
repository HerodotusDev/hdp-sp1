use core::{
    fmt::{Debug, Display},
    str::FromStr,
};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use thiserror_no_std::Error;

/// Enumeration representing supported Ethereum chain IDs.
#[derive(PartialEq, Eq, Clone, Copy, Hash)]
pub enum ChainId {
    /// Ethereum Mainnet - chain ID 1
    EthereumMainnet,
    /// Ethereum Sepolia - chain ID 11155111
    EthereumSepolia,
}

impl Default for ChainId {
    fn default() -> Self {
        Self::EthereumSepolia
    }
}

/// Error type for parsing [`ChainId`] from invalid inputs.
#[derive(Error, Debug, PartialEq)]
#[error("Failed to parse ChainId: {input}")]
pub struct ParseChainIdError {
    /// The invalid input that caused the parsing error.
    input: String,
}

impl TryFrom<u128> for ChainId {
    type Error = ParseChainIdError;

    fn try_from(value: u128) -> Result<Self, Self::Error> {
        Self::from_numeric_id(value)
    }
}

impl From<ChainId> for u128 {
    fn from(chain_id: ChainId) -> Self {
        chain_id.to_numeric_id()
    }
}

impl<'de> Deserialize<'de> for ChainId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        ChainId::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl Serialize for ChainId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl FromStr for ChainId {
    type Err = ParseChainIdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ETHEREUM_MAINNET" => Ok(Self::EthereumMainnet),
            "ETHEREUM_SEPOLIA" => Ok(Self::EthereumSepolia),

            _ => Err(ParseChainIdError {
                input: s.to_string(),
            }),
        }
    }
}

impl Display for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainId::EthereumMainnet => write!(f, "ETHEREUM_MAINNET"),
            ChainId::EthereumSepolia => write!(f, "ETHEREUM_SEPOLIA"),
        }
    }
}

impl Debug for ChainId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChainId::EthereumMainnet => write!(f, "ETHEREUM_MAINNET"),
            ChainId::EthereumSepolia => write!(f, "ETHEREUM_SEPOLIA"),
        }
    }
}

impl ChainId {
    /// Converts the typed ChainId enum into its numeric representation.
    /// This numeric ID is used for encoding in Solidity and Cairo.
    ///
    /// # Returns
    /// A u128 representing the numeric chain ID:
    /// - 1 for Ethereum Mainnet
    /// - 11155111 for Ethereum Sepolia
    pub fn to_numeric_id(&self) -> u128 {
        match self {
            ChainId::EthereumMainnet => 1,
            ChainId::EthereumSepolia => 11155111,
        }
    }

    /// Converts a numeric chain ID into its corresponding ChainId enum.
    /// This method is the reverse of `to_numeric_id()`.
    ///
    /// # Arguments
    /// * `id` - A u128 representing the numeric chain ID
    ///
    /// # Returns
    /// A Result containing the corresponding ChainId enum if successful,
    /// or a ParseChainIdError if the numeric ID is not recognized.
    pub fn from_numeric_id(id: u128) -> Result<Self, ParseChainIdError> {
        match id {
            1 => Ok(Self::EthereumMainnet),
            11155111 => Ok(Self::EthereumSepolia),
            i => Err(ParseChainIdError {
                input: i.to_string(),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        assert_eq!(
            ChainId::from_str("ETHEREUM_MAINNET").unwrap(),
            ChainId::EthereumMainnet
        );
        assert_eq!(
            ChainId::from_str("ETHEREUM_SEPOLIA").unwrap(),
            ChainId::EthereumSepolia
        );

        assert!(ChainId::from_str("INVALID").is_err());
    }

    #[test]
    fn test_display() {
        assert_eq!(ChainId::EthereumMainnet.to_string(), "ETHEREUM_MAINNET");
        assert_eq!(ChainId::EthereumSepolia.to_string(), "ETHEREUM_SEPOLIA");
    }

    #[test]
    fn test_to_numeric_id() {
        assert_eq!(ChainId::EthereumMainnet.to_numeric_id(), 1);
        assert_eq!(ChainId::EthereumSepolia.to_numeric_id(), 11155111);
    }

    #[test]
    fn test_from_numeric_id() {
        assert_eq!(ChainId::from_numeric_id(1), Ok(ChainId::EthereumMainnet));
        assert_eq!(
            ChainId::from_numeric_id(11155111),
            Ok(ChainId::EthereumSepolia)
        );

        assert!(ChainId::from_numeric_id(999).is_err());
    }
}
