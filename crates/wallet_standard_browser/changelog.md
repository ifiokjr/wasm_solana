# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
