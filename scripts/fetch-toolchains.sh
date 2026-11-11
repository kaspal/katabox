#!/usr/bin/env bash
# Dev/CI helper: install Python and JavaScript/TypeScript (runtimes and language servers)
# into the dev sandbox root, through the same code the app uses (kb-toolchain, ADR 0015).
# The pins, sizes and SHA-256 hashes live only in crates/kb-toolchain/manifest.toml.
# PHP and Rust are not available on Linux yet (ADR 0009).
#
# Usage: scripts/fetch-toolchains.sh [root] [--runtimes-only]   (default root: target/katabox)
#
# A root made by an older version of this script has no install receipts: it is cleaned
# and installed again once.
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="$script_dir/../target/katabox"
extra=()
for arg in "$@"; do
    case "$arg" in
        --runtimes-only) extra+=(--runtimes-only) ;;
        *) root="$arg" ;;
    esac
done
mkdir -p "$root"
root="$(cd "$root" && pwd -P)"

cd "$script_dir/.."
cargo run -q -p kb-toolchain --release -- install --root "$root" --create-root "${extra[@]}" python typescript
