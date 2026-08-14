#!/usr/bin/env bash
# Batch-transmute Mixxx engine headers → crates/vmp-vinyl/transmute_raw via v01d/libclang.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
MIXXX="${MIXXX_SRC:-$ROOT/../mixxx}"
OUT="$ROOT/crates/vmp-vinyl/transmute_raw"
mkdir -p "$OUT"

if [[ ! -d "$MIXXX/src/engine" ]]; then
  echo "Mixxx source not found at $MIXXX"
  echo "  git clone --depth 1 https://github.com/mixxxdj/mixxx.git $MIXXX"
  exit 1
fi

HEADERS=(
  enginebuffer.h
  enginemixer.h
  engineobject.h
  engine.h
  channelhandle.h
  enginedelay.h
  enginepregain.h
)

echo "FUTE clang-ast: Mixxx engine → $OUT"
for h in "${HEADERS[@]}"; do
  src="$MIXXX/src/engine/$h"
  if [[ ! -f "$src" ]]; then
    echo "  skip missing $h"
    continue
  fi
  base="${h%.h}"
  cargo run -q -p fute --bin v01d --features clang-ast -- lang "$src" \
    -o "$OUT/${base}.rs" --from cpp
done

echo "Done. Review $OUT and polish into crates/vmp-vinyl/src/"
