# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.7.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.6.1...wasm_client_solana@v0.7.0) - 2024-11-25

### <!-- 0 -->🎉 Added

- [**breaking**] upgrade to solana@v2 ([#20](https://github.com/ifiokjr/wasm_solana/pull/20))

## [0.6.1](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.6.0...wasm_client_solana@v0.6.1) - 2024-11-04

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove `unused import`

## [0.6.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.5.0...wasm_client_solana@v0.6.0) - 2024-10-20

### <!-- 0 -->🎉 Added

- [**breaking**] `SolanaRpcClient::get_nonce_account_*` methods

## [0.5.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.4.0...wasm_client_solana@v0.5.0) - 2024-10-13

### <!-- 0 -->🎉 Added

- [**breaking**] rename and add `authority` to `initialize_address_lookup_table`

### <!-- 1 -->🐛 Bug Fixes

- update instances of `pubkey` and `sign_message` after rename

## [0.4.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.6...wasm_client_solana@v0.4.0) - 2024-10-12

### <!-- 0 -->🎉 Added

- [**breaking**] add `async` methods directly to `VersionedTransactionExtension`
- add serialize and deserialize support to more method structs

## [0.3.6](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.5...wasm_client_solana@v0.3.6) - 2024-10-11

### <!-- 0 -->🎉 Added

- add `ToUiAccount` for converting from a `ReadableAccount` to a `UiAccount`
- support deserialize for `GetBalanceRequest`

## [0.3.5](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.4...wasm_client_solana@v0.3.5) - 2024-10-09

### <!-- 0 -->🎉 Added

- support deserialize `GetSignatureStatusesRequest`

## [0.3.4](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.3...wasm_client_solana@v0.3.4) - 2024-10-08

### <!-- 0 -->🎉 Added

- add `new_with_provider` method to `SolanaRpcClient`
- support deserialize `SimulateTransactionRequest`
- support deserialize `SendTransactionRequest`
- support deserialize `GetLatestBlockhashRequest`
- support `RpcProvider` in wasm environment
- support `ProgramTestContext` `RpcProvider`
- add `RpcProvider` trait

### <!-- 3 -->📚 Documentation

- add comment to `simulate_transaction` method

## [0.3.3](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.2...wasm_client_solana@v0.3.3) - 2024-10-07

### <!-- 0 -->🎉 Added

- implement `From<&SolanaRpcClient>` for `SolanaRpcClient`

## [0.3.2](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.1...wasm_client_solana@v0.3.2) - 2024-10-03

### <!-- 0 -->🎉 Added

- new `Unsubscription` struct which unsubscribes from rpc websocket methods

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update formatting

## [0.3.1](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.3.0...wasm_client_solana@v0.3.1) - 2024-09-28

### <!-- 0 -->🎉 Added

- support browsers for subscriptions
- automate `AbortController::abort` on wasm `Request` drop

### <!-- 1 -->🐛 Bug Fixes

- websocket url generator
- wasm `HttpProvider` only aborts if fetch is still pending

### <!-- 6 -->🧪 Testing

- make wasm tests work in browser
- initial basic wasm tests

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove unused deps

## [0.3.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_solana@v0.2.1...wasm_client_solana@v0.3.0) - 2024-09-21

### <!-- 0 -->🎉 Added

- make `Context::slot` public
- make `spawn_local` public
- [**breaking**] remove `&mut` requirement for all `*_subscribe` methods

### <!-- 1 -->🐛 Bug Fixes

- update builder methods for `LogsSubscribeRequest`
- resolve broken support for streams

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
