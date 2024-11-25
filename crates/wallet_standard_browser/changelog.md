# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.3.1...wallet_standard_browser@v0.4.0) - 2024-11-25

### <!-- 0 -->🎉 Added

- [**breaking**] upgrade to solana@v2 ([#20](https://github.com/ifiokjr/wasm_solana/pull/20))

## [0.3.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.3.0...wallet_standard_browser@v0.3.1) - 2024-10-13

### <!-- 1 -->🐛 Bug Fixes

- move `solana` specific features
- update instances of `pubkey` and `sign_message` after rename

## [0.3.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.2.1...wallet_standard_browser@v0.3.0) - 2024-10-12

### <!-- 2 -->🚜 Refactor

- [**breaking**] remove `AsyncSigner`

## [0.2.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.2.0...wallet_standard_browser@v0.2.1) - 2024-10-03

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update formatting

## [0.2.0](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.1.4...wallet_standard_browser@v0.2.0) - 2024-09-28

### <!-- 0 -->🎉 Added

- [**breaking**] make `signed_transaction` return `VersionedTransaction`

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove unused deps

## [0.1.4](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.1.3...wallet_standard_browser@v0.1.4) - 2024-09-21

### <!-- 3 -->📚 Documentation

- fix typo

## [0.1.3](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.1.2...wallet_standard_browser@v0.1.3) - 2024-09-18

### <!-- 3 -->📚 Documentation

- include crate `readme.md`

## [0.1.2](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.1.1...wallet_standard_browser@v0.1.2) - 2024-09-16

### <!-- 0 -->🎉 Added

- use `esm.sh` instead of node

### <!-- 1 -->🐛 Bug Fixes

- `copy:js` command for `app.js` and `wallet.js` wasm binding

### <!-- 2 -->🚜 Refactor

- remove unnecessary export

### <!-- 7 -->⚙️ Miscellaneous Tasks

- make crate versioning independent

## [0.1.1](https://github.com/ifiokjr/wasm_solana/compare/wallet_standard_browser@v0.1.0...wallet_standard_browser@v0.1.1) - 2024-09-13

### <!-- 3 -->📚 Documentation

- add `wallet-standard` github repo link
- update crate readme description

## [0.1.0](https://github.com/ifiokjr/wasm_solana/releases/tag/wallet_standard_browser@v0.1.0) - 2024-09-12

### <!-- 0 -->🎉 Added

- use `WalletSolana` for signing anchor transactions
- add memory based standard wallet implementation
- initial implementation of websockets
- initial commit

### <!-- 1 -->🐛 Bug Fixes

- import `DefaultHasher` from compatible path

### <!-- 3 -->📚 Documentation

- prepare for initial release

### <!-- 6 -->🧪 Testing

- passing tests for `wasm_client_solana`
