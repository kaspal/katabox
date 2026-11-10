#!/usr/bin/env bash
# Dev/CI helper: fetch a pinned, checksummed python-build-standalone build for the
# Linux sandbox attack suite. This is NOT the product download path (that lives in
# kb-toolchain and needs maintainer sign-off); it only feeds `cargo test -p kb-runner`.
#
# Also creates the dev sandbox root (target/katabox, marked with .katabox-sandbox-root),
# which mirrors the app layout: <root>/toolchains/<runtime> and <root>/runs/<run>.
# Linux counterpart of scripts/fetch-python-toolchain.ps1 (same release, same layout).
#
# Usage: scripts/fetch-python-toolchain.sh [root]   (default: target/katabox)
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="${1:-$script_dir/../target/katabox}"
mkdir -p "$root/runs"
root="$(cd "$root" && pwd -P)"
[ -f "$root/.katabox-sandbox-root" ] || : > "$root/.katabox-sandbox-root"
dest="$root/toolchains/python-3.14"

release="20261026"
# From the release's SHA256SUMS file.
case "$(uname -m)" in
    x86_64)
        asset="cpython-3.14.10+$release-x86_64-unknown-linux-gnu-install_only.tar.gz"
        sha256="1b38024924eb7f4cf0cab45d7b871d1f1991354190fccd2625ac7529a43786e2"
        ;;
    aarch64)
        asset="cpython-3.14.10+$release-aarch64-unknown-linux-gnu-install_only.tar.gz"
        sha256="dcc31ce7b9ddbbffac160da7fff0cc069054f43eca52382a7d0eebf94e985b88"
        ;;
    *)
        echo "unsupported architecture: $(uname -m)" >&2
        exit 1
        ;;
esac
url="https://github.com/astral-sh/python-build-standalone/releases/download/$release/${asset//+/%2B}"

if [ -x "$dest/bin/python3" ]; then
    echo "Toolchain already present at $dest"
    exit 0
fi

mkdir -p "$(dirname "$dest")"
work="$(mktemp -d "$(dirname "$dest")/_fetch-XXXXXXXX")"
trap 'rm -rf "$work"' EXIT

echo "Downloading $asset"
curl -fsSL --proto '=https' --tlsv1.2 -A katabox-dev -o "$work/python.tar.gz" "$url"

actual="$(sha256sum "$work/python.tar.gz" | cut -d' ' -f1)"
if [ "$actual" != "$sha256" ]; then
    printf 'SHA-256 mismatch for %s\n expected %s\n actual   %s\n' "$asset" "$sha256" "$actual" >&2
    exit 1
fi
echo "SHA-256 verified"

# Extract into an empty folder; refuse absolute paths and `..` members.
mkdir "$work/x"
if tar -tzf "$work/python.tar.gz" | grep -qE '(^/|(^|/)\.\.(/|$))'; then
    echo "archive contains unsafe paths" >&2
    exit 1
fi
tar -xzf "$work/python.tar.gz" -C "$work/x" --no-same-owner --no-same-permissions

# The install_only archive has a single top-level `python/` directory.
[ -x "$work/x/python/bin/python3" ] || { echo "python3 not found in archive" >&2; exit 1; }
rm -rf "$dest"
mv "$work/x/python" "$dest"
echo "Toolchain installed at $dest"
