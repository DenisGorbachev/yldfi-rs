use alloy_primitives::B256;
use derive_more::From;
use serde_with::{DeserializeFromStr, SerializeDisplay};
use solana_hash::Hash;
use std::{fmt, str::FromStr};
use thiserror::Error;

/// A 32-byte block hash encoded as hex on EVM networks or base58 on Solana.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, From, DeserializeFromStr, SerializeDisplay)]
pub enum PortfolioBlockHash {
    Evm(B256),
    Solana(Hash),
}

impl FromStr for PortfolioBlockHash {
    type Err = ParsePortfolioBlockHashError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with("0x") {
            value.parse().map(Self::Evm).map_err(Self::Err::Evm)
        } else {
            value.parse().map(Self::Solana).map_err(Self::Err::Solana)
        }
    }
}

impl fmt::Display for PortfolioBlockHash {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evm(hash) => write!(f, "{hash:#x}"),
            Self::Solana(hash) => hash.fmt(f),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParsePortfolioBlockHashError {
    #[error("invalid EVM block hash: {0}")]
    Evm(#[source] alloy_primitives::hex::FromHexError),
    #[error("invalid Solana block hash: {0}")]
    Solana(#[source] solana_hash::ParseHashError),
}
