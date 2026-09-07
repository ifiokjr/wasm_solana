# Security

This workspace is published to crates.io and is intended as client infrastructure, so supply-chain security is treated as a CI gate.

## Dependency policy

`deny.toml` (cargo-deny) enforces:

- **Licenses** — an explicit allow-list. Notably `LGPL-3.0-or-later` is allowed for `nacl`, which implements the Wallet Standard's experimental encryption scheme. The workspace crates themselves are `Unlicense` (public domain).
- **Bans** — wildcard dependencies are denied outright; duplicate crate versions are surfaced as warnings (the Agave family legitimately ships a few).
- **Sources** — crates may only come from crates.io; git sources must be explicitly allow-listed.

## Advisory scanning

`security:audit` checks `Cargo.lock` against RustSec on every CI run.

## Workflow hardening

`security:zizmor` audits all GitHub Actions workflows with a zero-findings policy (`.github/zizmor.yml`):

- every action pinned by commit SHA,
- `persist-credentials: false` on every checkout,
- workflow-level permissions limited to `contents: read`,
- Dependabot updates behind a 7-day cooldown.

## Secret scanning

The pre-commit hook runs `gitleaks protect --staged`. CI publishing uses trusted publishing (crates.io OIDC) rather than stored tokens — see [CI and Releases](./ci-and-releases.md).

## Runtime posture

- The client performs read/write JSON-RPC and pubsub only; no key material is ever handled by `wasm_client_solana` itself.
- Signing is delegated to Wallet Standard wallets; `memory_wallet` holds keypairs in process memory for tests only.
- The `ssr` transport uses rustls via reqwest (no vendored OpenSSL).
