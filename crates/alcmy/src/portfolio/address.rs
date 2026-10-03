//! Typed addresses for the EVM and Solana networks supported by the Portfolio API.

pub use alloy_primitives::Address as EvmAddress;
pub use solana_address::Address as SolanaAddress;

use serde_with::{DeserializeFromStr, SerializeDisplay};
use std::{fmt, str::FromStr};
use thiserror::Error;

/// A wallet or token address, encoded as EVM hex or Solana base58 in JSON.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DeserializeFromStr, SerializeDisplay)]
pub enum PortfolioAddress {
    Evm(EvmAddress),
    Solana(SolanaAddress),
}

impl From<EvmAddress> for PortfolioAddress {
    fn from(address: EvmAddress) -> Self {
        Self::Evm(address)
    }
}

impl From<SolanaAddress> for PortfolioAddress {
    fn from(address: SolanaAddress) -> Self {
        Self::Solana(address)
    }
}

impl FromStr for PortfolioAddress {
    type Err = ParsePortfolioAddressError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.starts_with("0x") {
            value.parse().map(Self::Evm).map_err(Self::Err::Evm)
        } else {
            value.parse().map(Self::Solana).map_err(Self::Err::Solana)
        }
    }
}

impl fmt::Display for PortfolioAddress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Evm(address) => write!(f, "{address:#x}"),
            Self::Solana(address) => address.fmt(f),
        }
    }
}

#[derive(Debug, Error)]
pub enum ParsePortfolioAddressError {
    #[error("invalid EVM address: {0}")]
    Evm(#[source] alloy_primitives::hex::FromHexError),
    #[error("invalid Solana address: {0}")]
    Solana(#[source] solana_address::error::ParseAddressError),
}
