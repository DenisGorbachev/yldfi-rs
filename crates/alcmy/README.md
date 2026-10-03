<p align="center">
  <img src="https://raw.githubusercontent.com/yldfi/yldfi-rs/main/logo-128.png" alt="yld_fi" width="128" height="128">
</p>
<h1 align="center">alcmy</h1>
<p align="center">
  Unofficial Rust client for the <a href="https://www.alchemy.com">Alchemy</a> API
</p>
<p align="center">
  <a href="https://crates.io/crates/alcmy"><img src="https://img.shields.io/crates/v/alcmy.svg" alt="crates.io"></a>
  <a href="https://github.com/yldfi/yldfi-rs/blob/main/crates/alcmy/LICENSE"><img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="MIT License"></a>
</p>

## Features

- **NFT API** - Ownership, metadata, floor prices, spam detection
- **Prices API** - Token prices by symbol/address, historical data
- **Portfolio API** - Multi-chain token balances and NFT holdings
- **Token API** - ERC-20 balances, metadata, allowances
- **Transfers API** - Historical transaction data
- **Debug API** - Transaction and block tracing
- **Trace API** - Parity-style tracing
- **Bundler API** - ERC-4337 Account Abstraction
- **Gas Manager API** - Gas sponsorship and policy management
- **Wallet API** - Smart wallet operations
- **Accounts API** - Authentication (email, passkey, JWT)
- **Notify API** - Webhook management
- **Beacon API** - Ethereum consensus layer
- **Solana DAS API** - Digital Asset Standard queries

## Installation

```toml
[dependencies]
alcmy = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Quick Start

```rust
use alcmy::{Client, Network};

#[tokio::main]
async fn main() -> Result<(), alcmy::Error> {
    let client = Client::new("your-api-key", Network::EthMainnet)?;

    // Get NFTs for an address
    let nfts = client.nft().get_nfts_for_owner("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").await?;
    println!("Found {} NFTs", nfts.total_count);

    // Get token balances
    let balances = client.token().get_token_balances("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").await?;
    println!("Found {} tokens", balances.token_balances.len());

    Ok(())
}
```

## Supported Networks

Ethereum, Polygon, Arbitrum, Optimism, Base, zkSync, Solana, and 20+ more networks.

```rust
use alcmy::Network;

let client = Client::new("api-key", Network::EthMainnet)?;
let client = Client::new("api-key", Network::Polygon)?;
let client = Client::new("api-key", Network::Arbitrum)?;
let client = Client::new("api-key", Network::Base)?;
let client = Client::new("api-key", Network::SolanaMainnet)?;
```

## API Examples

### NFT API

```rust
// Get NFTs for owner
let nfts = client.nft().get_nfts_for_owner("0x...").await?;

// Get NFT metadata
let nft = client.nft().get_nft_metadata("0xcontract", "1").await?;

// Check if address owns NFT from collection (built on getNFTsForOwner)
let is_holder = client.nft().is_holder_of_contract("0xwallet", "0xcontract").await?;

// Get floor price
let floor = client.nft().get_floor_price("0xcontract").await?;
```

Alchemy retired `getCollectionsForOwner`, `getCollectionMetadata`,
`isHolderOfContract`, `getSpamContracts`, `searchContractMetadata`,
`summarizeNFTAttributes`, `computeRarity`, `invalidateContract`,
`isAirdropNFT` and `getNFTSales` on 2026-09-30, and the corresponding methods
were removed. Use `get_contracts_for_owner`, `get_contract_metadata`
(`opensea_metadata`), `is_spam_contract` and `get_nfts_for_contract` instead.
`is_holder_of_contract` remains and is implemented on top of `getNFTsForOwner`.

The Transaction Simulation API (`alchemy_simulateAssetChanges`,
`alchemy_simulateAssetChangesBundle`, `alchemy_simulateExecution`,
`alchemy_simulateExecutionBundle`) was also retired on 2026-09-30 and the
`simulation` module was removed. Use `client.debug().trace_call(...)`
(`debug_traceCall`) instead.

Other Alchemy platform changes reflected here:

- The Signer `POST /signup` endpoint was turned off on 2026-06-18, so
  `accounts().signup()` was removed.
- `gas_manager().request_paymaster_and_data_v06/_v07`
  (`alchemy_requestPaymasterAndData`) are `#[deprecated]` because Alchemy marks
  the method "to be deprecated"; use `request_gas_and_paymaster_data_v06/_v07`
  (`alchemy_requestGasAndPaymasterAndData`).
- `trace().filter()`, `trace().get()` and `trace().raw_transaction()` return an
  error without sending a request on `polygon-mainnet`/`polygon-amoy`: Alchemy
  stopped serving `trace_filter`, `trace_get` and `trace_rawTransaction` there on
  2026-08-01 (Erigon to Bor migration). The other `trace_*` methods still work.

### Token API

```rust
// Get ERC-20 balances
let balances = client.token().get_token_balances("0x...").await?;

// Get token metadata
let metadata = client.token().get_token_metadata("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").await?;
println!("{} ({})", metadata.name.unwrap(), metadata.symbol.unwrap());

// Get allowance
let allowance = client.token().get_token_allowance("0xtoken", "0xowner", "0xspender").await?;
```

### Portfolio API

Fetch wallet balances and token metadata across networks with
`get_tokens_by_address`. The default options include native and ERC-20 tokens,
request metadata, and disable prices. Use `get_tokens_by_address_with_options`
to change those flags, include block metadata, or continue pagination.

```rust
use alcmy::{portfolio::TokensByAddressOptions, Client, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new("your-api-key", Network::EthMainnet)?;
    let networks: &[&str] = &["eth-mainnet", "base-mainnet"];
    let wallets = &[("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", networks)];
    let mut options = TokensByAddressOptions::default();

    loop {
        let response = client
            .portfolio()
            .get_tokens_by_address_with_options(wallets, &options)
            .await?;

        if let Some(error) = response.error {
            return Err(format!("{}: {:?}", error.message, error.partial_errors).into());
        }
        for token in response.data.tokens {
            if let Some(error) = token.error {
                return Err(format!("{} {:?}: {error}", token.network, token.token_address).into());
            }
            println!("{} {:?}: {} {:?}",
                token.network, token.token_address, token.token_balance, token.token_metadata);
        }

        match response.data.page_key {
            Some(page_key) => options.page_key = Some(page_key),
            None => break,
        }
    }
    Ok(())
}
```

Each call returns one page. Balances remain hex strings so callers can parse them
without losing precision. `token_address: None` identifies a native token.
Metadata and its individual fields can be absent, so validate the fields your
application requires before saving a complete snapshot.

HTTP 200 can include `response.error.partial_errors` alongside successful tokens.
The response preserves both. Retry failed networks in separate requests with a
bounded retry policy; a pagination cursor does not retry them. Per-token errors
are separate metadata or pricing failures. See Alchemy's
[endpoint documentation](https://www.alchemy.com/docs/data/portfolio-apis/portfolio-api-endpoints/portfolio-api-endpoints/get-tokens-by-address)
and [partial-failure guidance](https://www.alchemy.com/docs/reference/portfolio-apis#handling-partial-failures).

The incorrectly modeled `get_token_info` method and its `TokenInfoRequest`,
`TokenAddressInfo`, `TokenInfo`, and `TokenInfoResponse` types were removed. Use
`get_tokens_by_address` for wallet holdings, or `token().get_token_metadata` to
look up a token contract's metadata.

### Transfers API

```rust
use alcmy::transfers::AssetTransfersOptions;

// Get transfers from an address
let transfers = client.transfers().get_transfers_from("0x...").await?;

// Get transfers with options
let options = AssetTransfersOptions::from_address("0x...")
    .category(vec!["erc20", "erc721"])
    .with_metadata()
    .exclude_zero_value();
let transfers = client.transfers().get_asset_transfers(&options).await?;
```

### Debug API

```rust
// Trace a transaction
let trace = client.debug().trace_transaction("0xtxhash").await?;

// Trace a call
let call = TraceCallObject::new("0xfrom", "0xto").data("0xcalldata");
let trace = client.debug().trace_call(&call, "latest").await?;
```

### Bundler API (ERC-4337)

```rust
// Get supported entry points
let entry_points = client.bundler().supported_entry_points().await?;

// Estimate gas for UserOperation
let gas = client.bundler().estimate_user_operation_gas(&user_op, "0xEntryPoint").await?;

// Send UserOperation
let hash = client.bundler().send_user_operation(&user_op, "0xEntryPoint").await?;

// Get UserOperation receipt
let receipt = client.bundler().get_user_operation_receipt(&hash).await?;
```

### Notify (Webhooks) & Gas Manager (Gas Sponsorship) admin APIs

These APIs do not accept the app API key, and each uses its own credential:

- **Notify API** (dashboard: *Webhooks*) - the *Auth Token* shown via the
  AUTH TOKEN button at the top right of the dashboard
  [Webhooks page](https://dashboard.alchemy.com/webhooks) (sidebar Data -> Webhooks),
  sent as `X-Alchemy-Token`.
- **Gas Manager Admin API** (dashboard: *Gas Sponsorship*) - an *Access Key*
  created under Dashboard -> Security with Gas Manager (Gas Sponsorship)
  permissions (billing/team admins only, see
  [how to create access keys](https://www.alchemy.com/docs/how-to-create-access-keys)),
  sent as `Authorization: Bearer`.

```rust
let config = Config::new("your-api-key", Network::EthMainnet)
    .with_notify_token("your-notify-auth-token")
    .with_access_key("your-gas-manager-access-key");
let client = Client::with_config(config)?;
let webhooks = client.notify().list_webhooks().await?;
let policies = client.gas_manager().list_policies().await?;
```

`Client::from_env` reads them from `ALCHEMY_NOTIFY_TOKEN` and `ALCHEMY_ACCESS_KEY`.

### Beacon API

Available on `EthMainnet`, `EthSepolia` and `EthHolesky`
(`https://{network}beacon.g.alchemy.com/v2/{apiKey}/eth/v1/...`).

```rust
// Get genesis info
let genesis = client.beacon().get_genesis().await?;

// Get validators
let validators = client.beacon().get_validators("head").await?;

// Get block
let block = client.beacon().get_block("head").await?;
```

### Solana DAS API

```rust
// Get asset by ID
let asset = client.solana().get_asset("AssetId123...").await?;

// Get assets by owner
let assets = client.solana().get_assets_by_owner("WalletAddress...").await?;

// Search assets
let request = SearchAssetsRequest::new().owner("...").collection("...");
let results = client.solana().search_assets(&request).await?;
```

## Terms of Service

This is an **unofficial** client. By using this library, you agree to comply with [Alchemy's Terms of Service](https://www.alchemy.com/terms-conditions).

## Disclaimer

This crate is not affiliated with or endorsed by Alchemy.

## License

MIT
