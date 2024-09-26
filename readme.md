<p align="center">
  <a href="#">
    <img width="300" height="300" src="./setup/assets/logo.svg"  />
  </a>
</p>

<p align="center">
  solana development with rust based wasm
</p>

<br />

<p align="center">
  <a href="#getting-started"><strong>Getting Started</strong></a> ·
  <a href="#why"><strong>Why?</strong></a> ·
  <a href="#plans"><strong>Plans</strong></a> ·
  <a href="./docs/docs"><strong>Documentation</strong></a> ·
  <a href="./.github/contributing.md"><strong>Contributing</strong></a>
</p>

<br />

<p align="center">
  <a href="https://github.com/ifiokjr/wasm_solana/actions?query=workflow:ci">
    <img src="https://github.com/ifiokjr/wasm_solana/workflows/ci/badge.svg?branch=main" alt="Continuous integration badge for github actions" title="CI Badge" />
  </a>
</p>

<br />

## Description

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
