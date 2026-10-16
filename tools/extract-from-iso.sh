#!/usr/bin/env bash
#
# Extract the SSX 3 (USA) game executable from the disc image so Ghidra and the
# Rust tools can use it. The disc root contains SYSTEM.CNF and the boot ELF
# `SLUS_207.72`.
#
# Uses bsdtar (libarchive: built into macOS, `libarchive-tools` on Debian/
# Ubuntu, `libarchive` elsewhere); on macOS without it, mounts the image with
# hdiutil.
#
# Usage: tools/extract-from-iso.sh ["/path/to/SSX 3 (USA).iso"]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ISO="${1:-$REPO_ROOT/SSX 3 (USA).iso}"
OUT="$REPO_ROOT/orig"
# SLUS_207.72 of SSX 3 (USA), the only version this project supports.
EXPECTED_SHA256=1b49d05ca2793922180851b9e1ce9ae2291d61a7863565ac4e71f12e967af7bc

if [[ ! -f "$ISO" ]]; then
    echo "error: disc image not found: $ISO" >&2
    echo "Put your image at the repository root as 'SSX 3 (USA).iso' or pass its path." >&2
    exit 1
fi

mkdir -p "$OUT"

if command -v bsdtar >/dev/null 2>&1; then
    echo "Extracting from $ISO ..."
    bsdtar -xf "$ISO" -C "$OUT" SLUS_207.72 SYSTEM.CNF
elif command -v hdiutil >/dev/null 2>&1; then
    echo "Attaching $ISO ..."
    MOUNT_POINT="$(hdiutil attach -nobrowse -readonly "$ISO" 2>/dev/null \
        | awk -F'\t' '/\/Volumes\// {print $NF; exit}')"
    if [[ -z "${MOUNT_POINT:-}" || ! -d "$MOUNT_POINT" ]]; then
        echo "error: could not determine mount point" >&2
        exit 1
    fi
    cleanup() { hdiutil detach "$MOUNT_POINT" >/dev/null 2>&1 || true; }
    trap cleanup EXIT
    cp "$MOUNT_POINT/SLUS_207.72" "$OUT/SLUS_207.72"
    cp "$MOUNT_POINT/SYSTEM.CNF" "$OUT/SYSTEM.CNF"
else
    echo "error: need bsdtar (libarchive) to read the disc image" >&2
    exit 1
fi

if command -v sha256sum >/dev/null 2>&1; then
    SHA="$(sha256sum "$OUT/SLUS_207.72" | cut -d' ' -f1)"
else
    SHA="$(shasum -a 256 "$OUT/SLUS_207.72" | cut -d' ' -f1)"
fi
if [[ "$SHA" != "$EXPECTED_SHA256" ]]; then
    echo "error: orig/SLUS_207.72 has SHA-256 $SHA" >&2
    echo "       expected $EXPECTED_SHA256 (SSX 3, USA, SLUS-20772)." >&2
    echo "       Other regions or revisions are not supported." >&2
    exit 1
fi
echo "orig/SLUS_207.72 OK (SHA-256 matches SSX 3 USA)"
