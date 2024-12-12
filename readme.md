# wallet_standard

> The wasm compatible implementation of the [Wallet Standard](https://github.com/wallet-standard/wallet-standard) for [Solana](https://github.com/anza-xyz/wallet-standard).

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
| `wallet_standard_wallets` | 0.1.15  | Testing utilities for Solana programs                           |

### Crate Details

- **wallet_standard**: Provides the core wallet standard interface implementation for Solana. This includes transaction signing, message signing, and other wallet-related functionality.

- **wallet_standard_browser**: Browser-specific implementation of the wallet standard, allowing seamless integration with web applications. Includes JavaScript bindings and browser-specific wallet detection.

- **wallet_standard_wallets**: A collection of utilities to make testing Solana programs easier. Includes helpers for setting up test validators, creating test accounts, and managing test transactions.

See the individual crates for more information.

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
