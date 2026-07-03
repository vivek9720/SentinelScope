#!/usr/bin/env bash
set -euo pipefail

ROOT="${SRC:-$(pwd)}"
cd "$ROOT"
mkdir -p "$OUT"

targets=(packet_fuzzer ioc_fuzzer rules_fuzzer policy_fuzzer)

if command -v cargo-fuzz >/dev/null 2>&1; then
  for target in "${targets[@]}"; do
    cargo fuzz build "$target" --release
    built="$(find target -type f -name "$target" | head -n 1)"
    if [[ -z "$built" ]]; then
      echo "missing built target $target" >&2
      exit 1
    fi
    cp "$built" "$OUT/$target"
  done
else
  for target in "${targets[@]}"; do
    cargo build --manifest-path fuzz/Cargo.toml --bin "$target" --release
    built="target/release/$target"
    if [[ ! -f "$built" ]]; then
      built="fuzz/target/release/$target"
    fi
    cp "$built" "$OUT/$target"
  done
fi

for target in "${targets[@]}"; do
  corpus="fuzz/corpus/$target"
  if [[ -d "$corpus" ]]; then
    (cd "$corpus" && zip -q -r "$OUT/${target}_seed_corpus.zip" .)
  fi
done
