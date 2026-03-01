#!/usr/bin/env bash
set -euo pipefail

# Build once if binary missing.
if [ ! -f "targets/rust_atproto_dasl/adapter/target/release/atproto_dasl_adapter" ]; then
  cargo build --release --manifest-path targets/rust_atproto_dasl/adapter/Cargo.toml 1>/dev/null
fi

exec targets/rust_atproto_dasl/adapter/target/release/atproto_dasl_adapter
