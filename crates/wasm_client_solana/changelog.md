# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.1](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.2.0...wasm_client_solana@v0.2.1) - 2024-09-18

### <!-- 3 -->📚 Documentation

- include crate `readme.md`

## [0.2.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.1.0...wasm_client_solana@v0.2.0) - 2024-09-16

### <!-- 0 -->🎉 Added

- use native `Pubkey`, `Hash` and `Signature` types in structs

## [0.1.0](https://github.com/ifiokjr/wasm_solana/releases/tag/wasm_client_solana@v0.1.0) - 2024-09-12

### <!-- 0 -->🎉 Added

- use `WalletSolana` for signing anchor transactions
- add memory based standard wallet implementation
- add `Stream` support for solana client websockets
- add more websocket methods
- initial implementation of websockets
- initial commit

### <!-- 2 -->🚜 Refactor

- switch methods to use builder pattern

### <!-- 3 -->📚 Documentation

- prepare for initial release

### <!-- 5 -->🎨 Styling

- update lints
- update linting

### <!-- 6 -->🧪 Testing

- basic tests for `MemoryWallet` now succeed
- passing tests for `wasm_client_solana`
- write first tests
