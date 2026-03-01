#!/usr/bin/env bash
set -euo pipefail

BIN="targets/go_dasl/adapter/go_dasl_adapter"
if [ ! -f "$BIN" ]; then
  echo "go_dasl_adapter not built. Run: (cd targets/go_dasl/adapter && go build -o go_dasl_adapter .)" >&2
  exit 2
fi

exec "$BIN"
