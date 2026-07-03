#!/usr/bin/env bash
set -euo pipefail

ROOT="${SRC:-$(pwd)}"
cd "$ROOT"
mkdir -p "$OUT"

if [[ -f /usr/lib/libFuzzingEngine.a ]]; then
  FUZZ_ENGINE="/usr/lib/libFuzzingEngine.a"
elif [[ "${LIB_FUZZING_ENGINE:-}" == *.a && -f "${LIB_FUZZING_ENGINE}" ]]; then
  FUZZ_ENGINE="${LIB_FUZZING_ENGINE}"
else
  echo "missing libFuzzer archive; expected /usr/lib/libFuzzingEngine.a" >&2
  exit 1
fi

if command -v "${CXX:-clang++}" >/dev/null 2>&1; then
  export CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER="${CXX:-clang++}"
fi

targets=(packet_fuzzer ioc_fuzzer rules_fuzzer policy_fuzzer)

for target in "${targets[@]}"; do
  cargo rustc --manifest-path fuzz/Cargo.toml --locked --release --bin "$target" -- \
    -C "link-arg=${FUZZ_ENGINE}"

  built="fuzz/target/release/$target"
  if [[ ! -f "$built" ]]; then
    built="target/release/$target"
  fi
  if [[ ! -f "$built" ]]; then
    echo "missing built target $target" >&2
    exit 1
  fi
  cp "$built" "$OUT/$target"
done

for target in "${targets[@]}"; do
  corpus="fuzz/corpus/$target"
  if [[ -d "$corpus" ]]; then
    (cd "$corpus" && zip -q -r "$OUT/${target}_seed_corpus.zip" .)
  fi
done
