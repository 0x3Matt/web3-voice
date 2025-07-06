# Web3Voice Smart Contracts

This is a collection of NEAR smart contracts for the Web3Voice platform, a decentralized voice recording and NFT marketplace.

## Contracts

- **Voice Token** (`voice_token/`) - NEP-141 fungible token (VOICE) for platform governance and payments
- **Voice NFT** (`voice_nft/`) - NEP-171 non-fungible tokens for voice recordings
- **Marketplace** (`marketplace/`) - Trading platform for voice NFTs with auctions and fixed-price sales
- **DAO** (`dao/`) - Governance contract for platform decisions and parameter changes
- **Orchestrator** (`orchestrator/`) - System coordination and cross-contract operations

## How to Build Locally?

Install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
# Build all contracts
cargo near build

# Build a specific contract
cd voice_token && cargo near build
cd voice_nft && cargo near build
cd marketplace && cargo near build
cd dao && cargo near build
cd orchestrator && cargo near build
```

## How to Test Locally?

```bash
cargo test
```

## How to Deploy?

Deployment is automated with GitHub Actions CI/CD pipeline.
To deploy manually, install [`cargo-near`](https://github.com/near/cargo-near) and run:

```bash
cargo near deploy build-reproducible-wasm <account-id>
```

## Useful Links

- [cargo-near](https://github.com/near/cargo-near) - NEAR smart contract development toolkit for Rust
- [near CLI](https://near.cli.rs) - Interact with NEAR blockchain from command line
- [NEAR Rust SDK Documentation](https://docs.near.org/sdk/rust/introduction)
- [NEAR Documentation](https://docs.near.org)
- [NEAR StackOverflow](https://stackoverflow.com/questions/tagged/nearprotocol)
- [NEAR Discord](https://near.chat)
- [NEAR Telegram Developers Community Group](https://t.me/neardev)
- NEAR DevHub: [Telegram](https://t.me/neardevhub), [Twitter](https://twitter.com/neardevhub)
