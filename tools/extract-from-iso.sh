#!/usr/bin/env bash
#
# Extract the SSX 3 (USA) game executable from the disc image so Ghidra and the
# Rust tests can use it. The disc is a UDF filesystem whose root contains
# SYSTEM.CNF and the boot ELF `SLUS_207.72`.
#
# Usage: tools/extract-from-iso.sh ["/path/to/SSX 3 (USA).iso"]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ISO="${1:-$REPO_ROOT/SSX 3 (USA).iso}"
OUT="$REPO_ROOT/orig"

if [[ ! -f "$ISO" ]]; then
    echo "error: disc image not found: $ISO" >&2
    exit 1
fi

mkdir -p "$OUT"

echo "Attaching $ISO ..."
MOUNT_POINT="$(hdiutil attach -nobrowse -readonly "$ISO" 2>/dev/null \
    | awk -F'\t' '/\/Volumes\// {print $NF; exit}')"

if [[ -z "${MOUNT_POINT:-}" || ! -d "$MOUNT_POINT" ]]; then
    echo "error: could not determine mount point" >&2
    exit 1
fi

cleanup() { hdiutil detach "$MOUNT_POINT" >/dev/null 2>&1 || true; }
trap cleanup EXIT

cp -v "$MOUNT_POINT/SLUS_207.72" "$OUT/SLUS_207.72"
cp -v "$MOUNT_POINT/SYSTEM.CNF" "$OUT/SYSTEM.CNF"

echo
echo "SHA-256:"
shasum -a 256 "$OUT/SLUS_207.72"
