#!/bin/bash

# Exit with 1 if NEXTEST_ENV isn't defined.
if [ -z "$NEXTEST_ENV" ]; then
	exit 1
fi

# Set the default logging to `debug` for tests
echo "RUST_LOG=trace,tarpc=off,solana_metrics=off,solana_program_test=off,solana_accounts_db=off,solana_runtime=off" >>"$NEXTEST_ENV"
echo "RUST_LOG_SPAN_EVENTS=full" >>"$NEXTEST_ENV"
