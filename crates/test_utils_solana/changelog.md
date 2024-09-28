# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.4.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.3.0...test_utils_solana@v0.4.0) - 2024-09-28

### <!-- 1 -->🐛 Bug Fixes

- [**breaking**] remove `ssr` and `js` features since wasm doesn't actually work for `test_utils_solana`

### <!-- 6 -->🧪 Testing

- remove invalid feature in tests
- initial basic wasm tests

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove unused deps

## [0.3.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.2.1...test_utils_solana@v0.3.0) - 2024-09-21

### <!-- 0 -->🎉 Added

- [**breaking**] rename `external` module to `__private`

### <!-- 3 -->📚 Documentation

- fix typo

### <!-- 6 -->🧪 Testing

- `account_subscribe` and `logs_subscribe`

## [0.2.1](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.2.0...test_utils_solana@v0.2.1) - 2024-09-18

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wallet_standard, wasm_client_anchor, wasm_client_solana

## [0.2.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.1.1...test_utils_solana@v0.2.0) - 2024-09-16

### <!-- 0 -->🎉 Added

- add new fields to `TestValidatorRunnerProps`
- add `BanksClient` assertion macros
- [**breaking**] rename `BanksClientAnchorRequestMethods`

### <!-- 7 -->⚙️ Miscellaneous Tasks

- make crate versioning independent

## [0.1.0](https://github.com/ifiokjr/wasm_solana/releases/tag/test_utils_solana@v0.1.0) - 2024-09-12

### <!-- 0 -->🎉 Added

- use `WalletSolana` for signing anchor transactions
- add memory based standard wallet implementation
- add `Stream` support for solana client websockets
- initial implementation of websockets
- initial commit

### <!-- 2 -->🚜 Refactor

- update the name of anchor wallet trait

### <!-- 3 -->📚 Documentation

- add warning when using `namespace` with `TestValidatorRunnerProps`
- prepare for initial release

### <!-- 5 -->🎨 Styling

- lint all files

### <!-- 6 -->🧪 Testing

- basic tests for `MemoryWallet` now succeed
- passing tests for `wasm_client_solana`
- write first tests
