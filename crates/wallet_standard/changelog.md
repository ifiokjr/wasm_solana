# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.4.0...wallet_standard@v0.4.1) - 2024-11-25

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update Cargo.toml dependencies

## [0.4.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.3.0...wallet_standard@v0.4.0) - 2024-10-13

### <!-- 0 -->🎉 Added

- [**breaking**] rename `pubkey` to `WalletSolanaPubkey::solana_pubkey` to prevent clashes
- [**breaking**] rename `sign_message` to `WalletSolanaSignMessage::sign_message_async`

### <!-- 1 -->🐛 Bug Fixes

- update instances of `pubkey` and `sign_message` after rename

## [0.3.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.2.1...wallet_standard@v0.3.0) - 2024-10-12

### <!-- 2 -->🚜 Refactor

- [**breaking**] remove `AsyncSigner`

## [0.2.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.2.0...wallet_standard@v0.2.1) - 2024-10-03

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update formatting

## [0.2.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.1.3...wallet_standard@v0.2.0) - 2024-09-28

### <!-- 0 -->🎉 Added

- [**breaking**] make `signed_transaction` return `VersionedTransaction`

## [0.1.3](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.1.2...wallet_standard@v0.1.3) - 2024-09-18

### <!-- 3 -->📚 Documentation

- include crate `readme.md`

## [0.1.2](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.1.1...wallet_standard@v0.1.2) - 2024-09-16

### <!-- 2 -->🚜 Refactor

- `strip_option` methods to `SolanaSignAndSendTransactionProps`

### <!-- 7 -->⚙️ Miscellaneous Tasks

- make crate versioning independent

## [0.1.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard@v0.1.0...wallet_standard@v0.1.1) - 2024-09-13

### <!-- 2 -->🚜 Refactor

- remove unused imports

### <!-- 3 -->📚 Documentation

- add `wallet-standard` github repo link
- update crate readme description
