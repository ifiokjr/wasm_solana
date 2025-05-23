{ pkgs, lib, ... }:

{
  packages =
    [
      pkgs.binaryen
      pkgs.cargo-binstall
      pkgs.cargo-run-bin
      pkgs.chromedriver
      pkgs.curl
      pkgs.dprint
      pkgs.jq
      pkgs.libiconv
      pkgs.nixfmt-rfc-style
      pkgs.openssl
      pkgs.protobuf # needed for `solana-test-validator` in tests
      pkgs.rustup
      pkgs.shfmt
    ]
    ++ lib.optionals pkgs.stdenv.isDarwin [
			pkgs.apple-sdk_15
			pkgs.coreutils
		]
		++ lib.optionals pkgs.stdenv.isLinux [
			pkgs.libusb1
			pkgs.udev
			pkgs.systemd
		];

  # disable dotenv since it breaks the variable interpolation supported by `direnv`
  dotenv.disableHint = true;
	tasks = {
    "rustfmt:nightly" = {
      exec = "rustup toolchain install nightly --component rustfmt --force";
      before = [ "devenv:enterShell" ];
    };
  };

  scripts.anchor = {
    exec = ''
      set -e
      cargo bin anchor $@
    '';
    description = "The `anchor` executable";
		binary = "bash";
  };
  scripts."release-plz" = {
    exec = ''
      set -e
      cargo bin release-plz $@
    '';
    description = "The `release-plz` executable";
		binary = "bash";
  };
  scripts."wasm-bindgen-test-runner" = {
    exec = ''
      set -e
      cargo bin wasm-bindgen-test-runner $@
    '';
    description = "The `wasm-bindgen-test-runner` executable";
		binary = "bash";
  };
  scripts."install:all" = {
    exec = ''
      set -e
      install:cargo:bin
      install:solana
    '';
    description = "Install all packages.";
		binary = "bash";
  };
  scripts."generate:keypair" = {
    exec = ''
      set -e
      solana-keygen new -s -o $DEVENV_ROOT/$1.json --no-bip39-passphrase || true
    '';
    description = "Generate a local solana keypair. Must provide a name.";
		binary = "bash";
  };
  scripts."install:cargo:bin" = {
    exec = ''
      set -e
      cargo bin --install
    '';
    description = "Install cargo binaries locally.";
		binary = "bash";
  };
  scripts."update:deps" = {
    exec = ''
      set -e
      cargo update
      devenv update
    '';
    description = "Update dependencies.";
		binary = "bash";
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
		binary = "bash";
  };
  scripts."build:docs" = {
    exec = ''
      RUSTUP_TOOLCHAIN="nightly" RUSTDOCFLAGS="--cfg docsrs" cargo doc --workspace --exclude example_program --exclude example_client --exclude test_utils_solana --exclude test_utils_anchor
    '';
    description = "Build documentation site.";
		binary = "bash";
  };
  scripts."test:all" = {
    exec = ''
      set -e
      cargo test_memory_wallet_ssr
      cargo test_memory_wallet_docs
      cargo test_wasm_client_solana_ssr
      cargo test_wasm_client_solana_docs
      cargo test_streams
      cargo test_example_client
      test:validator
    '';
    description = "Run all tests across the crates";
		binary = "bash";
  };
  scripts."test:validator" = {
    exec = ''
      set -e
      validator:bg &
      pid=$!
      function cleanup {
        validator:kill
        kill -9 $pid
      }
      trap cleanup EXIT

      export WASM_BINDGEN_TEST_TIMEOUT=90

      cargo bin wait-for-them -t 10000 127.0.0.1:8899
      sleep 5
      echo "running tests in chrome..."
      CHROMEDRIVER=$DEVENV_DOTFILE/profile/bin/chromedriver cargo test_wasm
      # echo "running tests in firefox..."
      # GECKODRIVER=$DEVENV_DOTFILE/profile/bin/geckodriver cargo test_wasm
    '';
    description = "Run tests with a validator in the background.";
		binary = "bash";
  };
  scripts."coverage:all" = {
    exec = ''
      set -e
      cargo coverage_memory_wallet_ssr
      cargo coverage_memory_wallet_docs
      cargo coverage_wasm_client_solana_ssr
      cargo coverage_wasm_client_solana_docs
      cargo coverage_streams
      cargo coverage_example_client
      cargo coverage_codecov_report
    '';
    description = "Run coverage across the crates";
		binary = "bash";
  };
  scripts."fix:all" = {
    exec = ''
      set -e
      fix:clippy
      fix:format
    '';
    description = "Fix all autofixable problems.";
		binary = "bash";
  };
  scripts."fix:format" = {
    exec = ''
      set -e
      dprint fmt --config "$DEVENV_ROOT/dprint.json"
    '';
    description = "Format files with dprint.";
		binary = "bash";
  };
  scripts."fix:clippy" = {
    exec = ''
      set -e
      cargo clippy --fix --allow-dirty --allow-staged --all-features
    '';
    description = "Fix clippy lints for rust.";
		binary = "bash";
  };
  scripts."lint:all" = {
    exec = ''
      set -e
      lint:clippy
      lint:format
    '';
    description = "Run all checks.";
		binary = "bash";
  };
  scripts."lint:format" = {
    exec = ''
      set -e
      dprint check
    '';
    description = "Check that all files are formatted.";
		binary = "bash";
  };
  scripts."lint:clippy" = {
    exec = ''
      set -e
      cargo clippy --all-features
    '';
    description = "Check that all rust lints are passing.";
		binary = "bash";
  };
  scripts."validator:run" = {
    exec = ''
      set -e
      solana-test-validator --warp-slot 1000 --reset --quiet
    '';
    description = "Run the solana validator.";
		binary = "bash";
  };
  scripts."validator:bg" = {
    exec = ''
      set -e
      validator:kill
      validator:run
    '';
    description = "Run the solana validator in the background";
		binary = "bash";
  };
  scripts."validator:kill" = {
    exec = ''
      pids=$(lsof -i :8899 -t)

      if [ -n "$pids" ]; then
        kill $pids
        echo "Killed processes listening on port $port: $pids"
      else
        echo "No processes found listening on port $port"
      fi
    '';
    description = "Kill any running validator";
		binary = "bash";
  };
  scripts."setup:vscode" = {
    exec = ''
      set -e
      rm -rf .vscode
      cp -r $DEVENV_ROOT/setup/editors/vscode .vscode
    '';
    description = "Setup the environment for vscode.";
		binary = "bash";
  };
  scripts."setup:helix" = {
    exec = ''
      set -e
      rm -rf .helix
      cp -r $DEVENV_ROOT/setup/editors/helix .helix
    '';
    description = "Setup for the helix editor.";
		binary = "bash";
  };
  scripts."install:solana" = {
    exec = ''
      set -e
      SOLANA_DOWNLOAD_ROOT="https://github.com/anza-xyz/agave/releases/download"
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
		binary = "bash";
  };
}
