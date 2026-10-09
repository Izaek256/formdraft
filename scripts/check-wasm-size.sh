#!/usr/bin/env bash
set -euo pipefail

limit=$((25 * 1024 * 1024))

if [ $# -lt 1 ] || [ ! -e "$1" ]; then
  echo "check-wasm-size: no artifact yet (stub until S16)"
  exit 0
fi

target=$1
total=0
if [ -d "$target" ]; then
  while IFS= read -r f; do
    size=$(wc -c < "$f")
    total=$((total + size))
  done < <(find "$target" -name '*.wasm' -type f)
else
  total=$(wc -c < "$target")
fi

echo "check-wasm-size: ${total} bytes of WASM, limit ${limit}"
if [ "$total" -gt "$limit" ]; then
  echo "check-wasm-size: FAIL above 25 MiB (NFR-019)" >&2
  exit 1
fi
