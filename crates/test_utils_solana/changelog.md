# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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
