{ pkgs, lib, ... }:

{
  packages = [
    pkgs.binaryen
    pkgs.cargo-binstall
    pkgs.cargo-run-bin
    pkgs.coreutils
    pkgs.curl
    pkgs.dprint
    pkgs.fnm
    pkgs.jq
    pkgs.libiconv # needed for prisma to run
    pkgs.nixpkgs-fmt
    pkgs.protobuf # needed for `solana-test-validator` in tests
    pkgs.rustup
    pkgs.shfmt
    pkgs.taplo
    pkgs.trunk
    pkgs.wasm-pack
  ] ++ lib.optionals pkgs.stdenv.isDarwin (with pkgs.darwin.apple_sdk; [
    frameworks.Security
    frameworks.System
  ]);

  # disable dotenv since it breaks the variable interpolation supported by `direnv`
  dotenv.disableHint = true;

  scripts."install:all".exec = ''
    set -e
    install:pnpm
    setup:extensions
    install:cargo:bin
    install:solana
  '';
  scripts."generate:solana:keypair".exec = ''
    solana-keygen new -s -o $DEVENV_ROOT/.wallet.json --no-bip39-passphrase || true
  '';
  scripts."install:pnpm".exec = ''
    set -e

    if [ -f /.dockerenv ]; then
      yes | pnpm install
    elif [ -z "$CI" ]; then
      pnpm install
      echo "Installing playwright"
      pnpm playwright install
    else
      yes | pnpm install
    fi
  '';
  scripts."copy:js".exec = ''
    set -e
    pnpm esbuild --format=esm --target=es2021 --bundle "$DEVENV_ROOT/node_modules/@wallet-standard/app/src/index.ts" --outfile="$DEVENV_ROOT/crates/wallet_standard_browser/js/app.js"
    pnpm esbuild --format=esm --target=es2021 --bundle "$DEVENV_ROOT/node_modules/@wallet-standard/wallet/src/index.ts" --outfile="$DEVENV_ROOT/crates/wallet_standard_browser/js/wallet.js"
    fix:es
    fix:format
  '';
  scripts."install:cargo:bin".exec = ''
    cargo bin --install
  '';
  scripts."update:deps".exec = ''
    set -e
    cargo update
    pnpm update --latest --recursive -i
    copy:public
  '';
  scripts."fix:all".exec = ''
    set -e
    fix:clippy
    fix:es
    fix:format
  '';
  scripts."fix:format".exec = ''
    set -e
    dprint fmt --config "$DEVENV_ROOT/dprint.json"
  '';
  scripts."fix:clippy".exec = ''
    set -e
    cargo clippy --fix --allow-dirty --allow-staged --all-features
  '';
  scripts."fix:es".exec = ''
    set -e
    pnpm eslint --fix .
  '';
  scripts."lint:all".exec = ''
    set -e
    lint:clippy
    lint:es
    lint:format
  '';
  scripts."lint:format".exec = ''
    set -e
    dprint check
  '';
  scripts."lint:clippy".exec = ''
    set -e
    cargo clippy --all-features
  '';
  scripts."lint:es".exec = ''
    set -e
    pnpm eslint .
  '';
  scripts."setup:extensions".exec = ''
    set -e
    # tsx setup/scripts/extensions.ts
  '';
  scripts."setup:vscode".exec = ''
    set -e
    rm -rf .vscode
    cp -r $DEVENV_ROOT/setup/editors/vscode .vscode
  '';
  scripts."setup:helix".exec = ''
    set -e
    rm -rf .helix
    cp -r $DEVENV_ROOT/setup/editors/helix .helix
  '';
  scripts."setup:ci".exec = ''
    set -e
    # update github ci path
    echo "$DEVENV_PROFILE/bin" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/node_modules/.bin" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/solana-release/bin" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/pulumi" >> $GITHUB_PATH
    echo "$GITHUB_WORKSPACE/.local-cache/esc" >> $GITHUB_PATH

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

    fnm_env=$(fnm env --json)

    # Parse the JSON file contents
    PARSED_FNM_ENV=$(jq -r '.' <<< "$fnm_env")
    FNM_MULTISHELL_PATH=$(jq -r '.FNM_MULTISHELL_PATH' <<< "$PARSED_FNM_ENV")

    # Add fnm to the path
    echo "$FNM_MULTISHELL_PATH/bin" >> $GITHUB_PATH

    # add fnm environment variables
    for key in $(jq -r 'keys[]' <<< "$PARSED_FNM_ENV"); do
      value=$(jq -r ".$key" <<< "$PARSED_FNM_ENV")
      echo "$key=$value" >> $GITHUB_ENV
    done
  '';
  scripts."setup:docker".exec = ''
    set -e
    # update path
    echo "export PATH=$DEVENV_PROFILE/bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/node_modules/.bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/solana-release/bin:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/pulumi:\$PATH" >> /etc/profile
    echo "export PATH=$DEVENV_ROOT/.local-cache/esc:\$PATH" >> /etc/profile

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

    fnm_env=$(fnm env --json)

    # Parse the JSON file contents
    PARSED_FNM_ENV=$(jq -r '.' <<< "$fnm_env")
    FNM_MULTISHELL_PATH=$(jq -r '.FNM_MULTISHELL_PATH' <<< "$PARSED_FNM_ENV")

    # add fnm to the path
    echo "export PATH=$FNM_MULTISHELL_PATH/bin:\$PATH" >> /etc/profile

    # add fnm environment variables
    for key in $(jq -r 'keys[]' <<< "$PARSED_FNM_ENV"); do
      value=$(jq -r ".$key" <<< "$PARSED_FNM_ENV")
      echo "export $key=$value" >> /etc/profile
    done
  '';
  scripts."install:solana".exec = ''
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
}
