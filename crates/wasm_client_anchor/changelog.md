# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.2.4](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.2.3...wasm_client_anchor@v0.2.4) - 2024-10-05

### <!-- 0 -->🎉 Added

- add `transaction` method

## [0.2.3](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.2.2...wasm_client_anchor@v0.2.3) - 2024-10-04

### <!-- 0 -->🎉 Added

- add new `Into*` trait for anchor client programs

## [0.2.2](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.2.1...wasm_client_anchor@v0.2.2) - 2024-10-03

### <!-- 7 -->⚙️ Miscellaneous Tasks

- update formatting

## [0.2.1](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.2.0...wasm_client_anchor@v0.2.1) - 2024-09-28

### <!-- 0 -->🎉 Added

- add `create_program_client` macro and test usage
- add `create_program_client_macro` for implementing request methods for anchor clients
- include `WalletAnchor` in prelude

### <!-- 1 -->🐛 Bug Fixes

- export and use internal `async_trait` in macros
- rename `signed_versioned_transaction` to `signed_transaction`

### <!-- 7 -->⚙️ Miscellaneous Tasks

- remove unused deps

## [0.2.0](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.1.3...wasm_client_anchor@v0.2.0) - 2024-09-21

### <!-- 0 -->🎉 Added

- support subscription to `emit!` anchor `Event`
- [**breaking**] rename `external` module to `__private`

## [0.1.3](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.1.2...wasm_client_anchor@v0.1.3) - 2024-09-18

### <!-- 3 -->📚 Documentation

- include crate `readme.md`

## [0.1.2](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.1.1...wasm_client_anchor@v0.1.2) - 2024-09-16

### <!-- 7 -->⚙️ Miscellaneous Tasks

- make crate versioning independent

## [0.1.1](https://github.com/ifiokjr/wasm_solana/compare/wasm_client_anchor@v0.1.0...wasm_client_anchor@v0.1.1) - 2024-09-13

### <!-- 0 -->🎉 Added

- add `wallet_standard` to `external` exports

### <!-- 1 -->🐛 Bug Fixes

- remove duplication from broken macro

### <!-- 3 -->📚 Documentation

- update crate readme description

## [0.1.0](https://github.com/ifiokjr/wasm_solana/releases/tag/wasm_client_anchor@v0.1.0) - 2024-09-12

### <!-- 0 -->🎉 Added

- use `WalletSolana` for signing anchor transactions
- add `Stream` support for solana client websockets
- initial implementation of websockets
- initial commit

### <!-- 2 -->🚜 Refactor

- update the name of anchor wallet trait

### <!-- 3 -->📚 Documentation

- remove references to unrelated projects
- prepare for initial release

### <!-- 5 -->🎨 Styling

- update lints
