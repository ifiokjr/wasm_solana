{ pkgs, lib, ... }:

{
  packages =
    [
      pkgs.act
      pkgs.binaryen
      pkgs.cargo-binstall
      pkgs.cargo-run-bin
      pkgs.chromedriver
      pkgs.coreutils
      pkgs.curl
      pkgs.deno
      pkgs.dprint
      pkgs.jq
      pkgs.libiconv
      pkgs.nixfmt-rfc-style
      pkgs.protobuf # needed for `solana-test-validator` in tests
      pkgs.rustup
      pkgs.shfmt
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin (
      with pkgs.darwin.apple_sdk;
      [
        frameworks.CoreFoundation
        frameworks.Security
        frameworks.System
        frameworks.SystemConfiguration
      ]
    );

  # disable dotenv since it breaks the variable interpolation supported by `direnv`
  dotenv.disableHint = true;

  scripts.anchor = {
    exec = ''
      set -e
      cargo bin anchor $@
    '';
    description = "The `anchor` executable";
  };
  scripts."release-plz" = {
    exec = ''
      set -e
      cargo bin release-plz $@
    '';
    description = "The `release-plz` executable";
  };
  scripts."install:all" = {
    exec = ''
      set -e
      install:cargo:bin
      install:solana
    '';
    description = "Install all packages.";
  };
  scripts."generate:keypair" = {
    exec = ''
      set -e
      solana-keygen new -s -o $DEVENV_ROOT/$1.json --no-bip39-passphrase || true
    '';
    description = "Generate a local solana keypair. Must provide a name.";
  };
  scripts."install:cargo:bin" = {
    exec = ''
      set -e
      cargo bin --install
    '';
    description = "Install cargo binaries locally.";
  };
  scripts."copy:js" = {
    exec = ''
      set -e
      curl -L https://esm.sh/v135/@wallet-standard/app@1/es2022/app.development.mjs -o $DEVENV_ROOT/crates/wallet_standard_browser/js/app.js
      curl -L https://esm.sh/v135/@wallet-standard/wallet@1/es2022/wallet.development.mjs -o "$DEVENV_ROOT/crates/wallet_standard_browser/js/wallet.js"
      dprint fmt "./crates/wallet_standard_browser/js/*.js"
    '';
    description = "Copy the JS needed for the `wallet_standard_browser`.";
  };
  scripts."update:deps" = {
    exec = ''
      set -e
      cargo update
      devenv update
      copy:js
    '';
    description = "Update dependencies.";
  };
  scripts."build:all" = {
    exec = ''
      set -e
      if [ -z "$CI" ]; then
        echo "Builing project locally"
        cargo build --all-features
      else
        echo "Building in CI"
        cargo build --all-features --locked
      fi
    '';
    description = "Build all crates with all features activated.";
  };
  scripts."build:docs" = {
    exec = ''
      RUSTDOCFLAGS="--cfg docsrs" cargo doc --all-features --workspace --exclude example_program --exclude test_utils --exclude example_client
    '';
    description = "Build documentation site.";
  };
  scripts."test:all" = {
    exec = ''
      set -e
      cargo test_wallet_standard_wallets_ssr
      cargo test_wallet_standard_wallets_docs
      cargo test_wasm_client_solana_ssr
      cargo test_wasm_client_solana_docs
      cargo test_streams
      cargo test_example_client
    '';
    description = "Run all tests across the crates";
  };
  scripts."coverage:all" = {
    exec = ''
      set -e
      cargo coverage_wallet_standard_wallets_ssr
      cargo coverage_wallet_standard_wallets_docs
      cargo coverage_wasm_client_solana_ssr
      cargo coverage_wasm_client_solana_docs
      cargo coverage_streams
      cargo coverage_example_client
      cargo coverage_codecov_report
    '';
    description = "Run coverage across the crates";
  };
  scripts."fix:all" = {
    exec = ''
      set -e
      fix:clippy
      fix:format
    '';
    description = "Fix all autofixable problems.";
  };
  scripts."fix:format" = {
    exec = ''
      set -e
      dprint fmt --config "$DEVENV_ROOT/dprint.json"
    '';
    description = "Format files with dprint.";
  };
  scripts."fix:clippy" = {
    exec = ''
      set -e
      cargo clippy --fix --allow-dirty --allow-staged --all-features
    '';
    description = "Fix clippy lints for rust.";
  };
  scripts."lint:all" = {
    exec = ''
      set -e
      lint:clippy
      lint:format
    '';
    description = "Run all checks.";
  };
  scripts."lint:format" = {
    exec = ''
      set -e
      dprint check
    '';
    description = "Check that all files are formatted.";
  };
  scripts."lint:clippy" = {
    exec = ''
      set -e
      cargo clippy --all-features
    '';
    description = "Check that all rust lints are passing.";
  };
  scripts."validator:run" = {
    exec = ''
      set -e
      solana-test-validator --warp-slot 1000
    '';
  };
  scripts."setup:vscode" = {
    exec = ''
      set -e
      rm -rf .vscode
      cp -r $DEVENV_ROOT/setup/editors/vscode .vscode
    '';
    description = "Setup the environment for vscode.";
  };
  scripts."setup:helix" = {
    exec = ''
      set -e
      rm -rf .helix
      cp -r $DEVENV_ROOT/setup/editors/helix .helix
    '';
    description = "Setup for the helix editor.";
  };
  scripts."setup:ci" = {
    exec = ''
      set -e
      # update github ci path
      echo "$DEVENV_PROFILE/bin" >> $GITHUB_PATH
      echo "$GITHUB_WORKSPACE/.local-cache/solana-release/bin" >> $GITHUB_PATH

      # update github ci environment
      echo "DEVENV_PROFILE=$DEVENV_PROFILE" >> $GITHUB_ENV

      # prepend common compilation lookup paths
      echo "PKG_CONFIG_PATH=$PKG_CONFIG_PATH" >> $GITHUB_ENV
      echo "LD_LIBRARY_PATH=$LD_LIBRARY_PATH" >> $GITHUB_ENV
      echo "LIBRARY_PATH=$LIBRARY_PATH" >> $GITHUB_ENV
      echo "C_INCLUDE_PATH=$C_INCLUDE_PATH" >> $GITHUB_ENV

      # these provide shell completions / default config options
      echo "XDG_DATA_DIRS=$XDG_DATA_DIRS" >> $GITHUB_ENV
      echo "XDG_CONFIG_DIRS=$XDG_CONFIG_DIRS" >> $GITHUB_ENV

      echo "DEVENV_DOTFILE=$DEVENV_DOTFILE" >> $GITHUB_ENV
      echo "DEVENV_PROFILE=$DEVENV_PROFILE" >> $GITHUB_ENV
      echo "DEVENV_ROOT=$DEVENV_ROOT" >> $GITHUB_ENV
      echo "DEVENV_STATE=$DEVENV_STATE" >> $GITHUB_ENV

      echo "CHROMEDRIVER=$DEVENV_DOTFILE/profile/bin/chromedriver" >> $GITHUB_ENV
      # Temporary fix for a bug in anchor@0.30.1 https://github.com/coral-xyz/anchor/issues/3042
      echo "ANCHOR_IDL_BUILD_PROGRAM_PATH=$DEVENV_ROOT/programs/example_program" >> $GITHUB_ENV
    '';
    description = "Setup devenv for GitHub Actions";
  };
  scripts."setup:docker" = {
    exec = ''
      set -e
      # update path
      echo "export PATH=$DEVENV_PROFILE/bin:\$PATH" >> /etc/profile
      echo "export PATH=$DEVENV_ROOT/.local-cache/solana-release/bin:\$PATH" >> /etc/profile

      echo "export DEVENV_PROFILE=$DEVENV_PROFILE" >> /etc/profile
      echo "export PKG_CONFIG_PATH=$PKG_CONFIG_PATH" >> /etc/profile
      echo "export LD_LIBRARY_PATH=$LD_LIBRARY_PATH" >> /etc/profile
      echo "export LIBRARY_PATH=$LIBRARY_PATH" >> /etc/profile
      echo "export C_INCLUDE_PATH=$C_INCLUDE_PATH" >> /etc/profile
      echo "export XDG_DATA_DIRS=$XDG_DATA_DIRS" >> /etc/profile
      echo "export XDG_CONFIG_DIRS=$XDG_CONFIG_DIRS" >> /etc/profile

      echo "export DEVENV_DOTFILE=$DEVENV_DOTFILE" >> /etc/profile
      echo "export DEVENV_PROFILE=$DEVENV_PROFILE" >> /etc/profile
      echo "export DEVENV_ROOT=$DEVENV_ROOT" >> /etc/profile
      echo "export DEVENV_STATE=$DEVENV_STATE" >> /etc/profile

      echo "export CHROMEDRIVER=$DEVENV_DOTFILE/profile/bin/chromedriver" >> /etc/profile
      # Temporary fix for a bug in anchor@0.30.1 https://github.com/coral-xyz/anchor/issues/3042
      echo "export ANCHOR_IDL_BUILD_PROGRAM_PATH=$DEVENV_ROOT/programs/example_program" >> /etc/profile
    '';
    description = "Setup devenv shell for docker.";
  };
  scripts."install:solana" = {
    exec = ''
      set -e
      SOLANA_DOWNLOAD_ROOT="https://github.com/solana-labs/solana/releases/download"
      LOCAL_CACHE="$DEVENV_ROOT/.local-cache"
      VERSION=`cat $DEVENV_ROOT/setup/cache-versions.json | jq -r '.solana'`
      OS_TYPE="$(uname -s)"
      CPU_TYPE="$(uname -m)"

      case "$OS_TYPE" in
        Linux)
          OS_TYPE=unknown-linux-gnu
          ;;
        Darwin)
          if [[ $CPU_TYPE = arm64 ]]; then
            CPU_TYPE=aarch64
          fi
          OS_TYPE=apple-darwin
          ;;
        *)
          err "machine architecture is currently unsupported"
          ;;
      esac
      TARGET="$CPU_TYPE-$OS_TYPE"

      mkdir -p $LOCAL_CACHE
      TARBALL_PATH=solana-release-$TARGET.tar.bz2
      LOCAL_TARBALL_PATH=solana-$VERSION-release-$TARGET.tar.bz2
      FULL_TARBALL_PATH="$LOCAL_CACHE/$LOCAL_TARBALL_PATH"
      if [[ -e $FULL_TARBALL_PATH ]]
      then
        echo "Using cached solana release"
      else
        DOWNLOAD_URL="$SOLANA_DOWNLOAD_ROOT/$VERSION/$TARBALL_PATH"
        echo "Downloading $DOWNLOAD_URL to the local cache $FULL_TARBALL_PATH"
        curl --header "Authorization: Bearer $TEST_GITHUB_ACCESS_TOKEN" -sSfL "$DOWNLOAD_URL" -O
        mv $TARBALL_PATH $FULL_TARBALL_PATH
        tar jxf $FULL_TARBALL_PATH -C $LOCAL_CACHE
      fi
    '';
    description = "Install the version of solana or use one from the cache.";
  };
}
