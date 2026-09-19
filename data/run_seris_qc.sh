#!/bin/sh
set -eu

RAW="${1:-data/raw/seris/seris_canonical.csv}"
MANIFEST="${2:-data/manifests/seris_canonical.toml}"

if [ ! -f "$RAW" ]; then
  echo "ERROR: authorised SERIS raw file not found: $RAW" >&2
  echo "Follow data/acquire_seris.md; do not substitute synthetic or unrelated data." >&2
  exit 2
fi
if [ ! -f "$MANIFEST" ]; then
  echo "ERROR: resolved dataset manifest not found: $MANIFEST" >&2
  echo "Create it from data/manifests/seris_canonical_pending.toml using provider metadata." >&2
  exit 2
fi

echo "Authorised inputs found."
echo "Rust ingestion/QC entrypoint is the repository weather module; run cargo test first."
cargo test
echo "NOTE: dataset CLI binding remains to be added; do not claim dataset QC execution until the raw-file runner exists."
