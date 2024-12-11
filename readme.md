<p align="center">
  <a href="#">
    <img width="300" height="300" src="./setup/assets/logo.svg"  />
  </a>
</p>

<p align="center">
  solana development with a rust based wasm client
</p>

<p align="center">
  <a href="https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci">
    <img src="https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg" alt="Continuous integration badge for github actions" title="CI Badge" />
  </a>
</p>

<br />

## Description

This repository contains several crates that make it easier to interact with Solana in WebAssembly environments:

| Crate                     | Version | Description                                                     |
| ------------------------- | ------- | --------------------------------------------------------------- |
| `wallet_standard`         | 0.4.0   | Core implementation of the wallet standard interface for Solana |
| `wallet_standard_browser` | 0.3.1   | Browser-specific implementation of the wallet standard          |
| `test_utils_solana`       | 0.5.5   | Testing utilities for Solana programs                           |
| `test_utils_anchor`       | 0.5.5   | Testing utilities specific to Anchor programs                   |
| `wasm_client_anchor`      | 0.7.0   | WebAssembly client for interacting with Anchor programs         |
| `wasm_client_solana`      | 0.7.0   | WebAssembly client for interacting with Solana programs         |

### Crate Details

- **wallet_standard**: Provides the core wallet standard interface implementation for Solana. This includes transaction signing, message signing, and other wallet-related functionality.

- **wallet_standard_browser**: Browser-specific implementation of the wallet standard, allowing seamless integration with web applications. Includes JavaScript bindings and browser-specific wallet detection.

- **test_utils_solana**: A collection of utilities to make testing Solana programs easier. Includes helpers for setting up test validators, creating test accounts, and managing test transactions.

- **test_utils_anchor**: Extension of test utilities specifically designed for testing Anchor programs. Provides additional helpers for working with Anchor IDLs and program testing.

- **wasm_client_anchor**: A WebAssembly client for interacting with Anchor programs. Provides a type-safe interface for program interactions compiled to WebAssembly.

- **wasm_client_solana**: A WebAssembly client for general Solana program interactions. Includes methods for account management, transaction building, and RPC interactions.

See the individual crates for more information.

### scripts

- `anchor`: The `anchor` executable
- `build:all`: Build all crates with all features activated.
- `build:docs`: Build documentation site.
- `copy:js`: Copy the JS needed for the `wallet_standard_browser`.
- `coverage:all`: Run coverage across the crates
- `fix:all`: Fix all autofixable problems.
- `fix:clippy`: Fix clippy lints for rust.
- `fix:es`: Fix lints for JS / TS.
- `fix:format`: Format files with dprint.
- `generate:keypair`: Generate a local solana keypair. Must provide a name.
- `install:all`: Install all packages.
- `install:cargo:bin`: Install cargo binaries locally.
- `install:solana`: Install the version of solana or use one from the cache.
- `lint:all`: Run all checks.
- `lint:clippy`: Check that all rust lints are passing.
- `lint:es`: Check lints for all JS / TS files.
- `lint:format`: Check that all files are formatted.
- `release-plz`: The `release-plz` executable
- `setup:ci`: Setup devenv for GitHub Actions
- `setup:docker`: Setup devenv shell for docker.
- `setup:helix`: Setup for the helix editor.
- `setup:vscode`: Setup the environment for vscode.
- `test:all`: Run all tests across the crates
- `update:deps`: Update dependencies.

## Contributing

[`devenv`](https://devenv.sh/) is used to provide a reproducible development environment for this project. Follow the [getting started instructions](https://devenv.sh/getting-started/).

To automatically load the environment you should [install direnv](https://devenv.sh/automatic-shell-activation/) and then load the `direnv`.

```bash
# The security mechanism didn't allow to load the `.envrc`.
# Since we trust it, let's allow it execution.
direnv allow .
```

At this point you should see the `nix` commands available in your terminal. Any changes made to the `.envrc` file will require you to run the above command again.

Run the following commands to install all the required dependencies.

```bash
install:all
```

This installs all the node dependencies, cargo binaries and solana tooling locally so you don't need to worry about polluting your global namespace.

### Upgrading `devenv`

If you have an outdated version of `devenv` you can update it by running the following commands. If you have an easier way, please create a PR and I'll update these docs.

```bash
nix profile list # find the <index> of the devenv package
nix profile upgrade <index>
```

### Editor Setup

To setup recommended configuration for your favorite editor run the following commands.

```bash
setup:vscode # Setup vscode
```

## License

Unlicense, see the [LICENSE](./license) file.
