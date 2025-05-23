# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.3](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.7.2...test_utils_solana@v0.7.3) - 2025-03-14

### <!-- 0 -->🎉 Added

- update anchor dependencies and improve configuration

### <!-- 5 -->🎨 Styling

- update formatting

## [0.7.2](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.7.1...test_utils_solana@v0.7.2) - 2025-01-17

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update Cargo.lock dependencies

## [0.7.1](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.7.0...test_utils_solana@v0.7.1) - 2025-01-16

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update Cargo.lock dependencies

## [0.7.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.6.3...test_utils_solana@v0.7.0) - 2024-12-21

### <!-- 0 -->🎉 Added

- [**breaking**] add ports field to `TestValidatorRunnerProps`

## [0.6.3](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.6.2...test_utils_solana@v0.6.3) - 2024-12-13

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update Cargo.lock dependencies

## [0.6.2](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.6.1...test_utils_solana@v0.6.2) - 2024-12-13

### <!-- 0 -->🎉 Added

- add `test_utils_keypairs` crate
- replace `test_utils` with `test_utils_insta`
- add keypair utilities for testing

## [0.6.1](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.6.0...test_utils_solana@v0.6.1) - 2024-12-12

### <!-- 0 -->🎉 Added

- add trait method`wallet_sign_and_simulate_transaction` back

## [0.6.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.5...test_utils_solana@v0.6.0) - 2024-12-12

### <!-- 0 -->🎉 Added

- [**breaking**] add test_utils_anchor crate for testing anchor programs in wasm environments; update dependencies and configurations
- [**breaking**] upgrade to solana@v2 (#20)

## [0.5.5](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.4...test_utils_solana@v0.5.5) - 2024-11-04

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wasm_client_anchor, wasm_client_solana

## [0.5.4](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.3...test_utils_solana@v0.5.4) - 2024-10-20

### <!-- 1 -->🐛 Bug Fixes

- `TestRpcProvider` `simulate_transaction`

## [0.5.3](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.2...test_utils_solana@v0.5.3) - 2024-10-19

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wasm_client_anchor

## [0.5.2](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.1...test_utils_solana@v0.5.2) - 2024-10-13

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wallet_standard, wasm_client_anchor, wasm_client_solana

## [0.5.1](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.5.0...test_utils_solana@v0.5.1) - 2024-10-12

### <!-- 0 -->🎉 Added

- add `getMultipleAccounts` support to `TestRpcProvider`

## [0.5.0](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.7...test_utils_solana@v0.5.0) - 2024-10-11

### <!-- 0 -->🎉 Added

- support `getBalance` rpc method in `TestRpcProvider`

### <!-- 2 -->🚜 Refactor

- [**breaking**] remove `BanksClientAnchorRequestMethods` trait

## [0.4.7](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.6...test_utils_solana@v0.4.7) - 2024-10-09

### <!-- 0 -->🎉 Added

- support `getSignatureStatuses` in `TestRpcProvider`

## [0.4.6](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.5...test_utils_solana@v0.4.6) - 2024-10-08

### <!-- 0 -->🎉 Added

- add `TestRpcProvider::to_rpc_client` method to create a `SolanaRpcClient`

## [0.4.5](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.4...test_utils_solana@v0.4.5) - 2024-10-08

### <!-- 0 -->🎉 Added

- add new factory methods to `TestRpcProvider`
- support `ProgramTestContext` `RpcProvider`
- add `simulate_banks_client_transaction` method

### <!-- 1 -->🐛 Bug Fixes

- use `get_latest_blockhash` in simulation

## [0.4.4](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.3...test_utils_solana@v0.4.4) - 2024-10-07

### <!-- 0 -->🎉 Added

- add `get_anchor_account` method to `BanksClient`

## [0.4.3](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.2...test_utils_solana@v0.4.3) - 2024-10-05

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wasm_client_anchor

## [0.4.2](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.1...test_utils_solana@v0.4.2) - 2024-10-04

### <!-- 7 -->⚙️ Miscellaneous Tasks

- updated the following local packages: wasm_client_anchor

## [0.4.1](https://github.com/ifiokjr/wasm_solana/compare/test_utils_solana@v0.4.0...test_utils_solana@v0.4.1) - 2024-10-03

### <!-- 0 -->🎉 Added

- new `Unsubscription` struct which unsubscribes from rpc websocket methods

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update formatting

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
