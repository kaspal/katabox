#!/usr/bin/env bash
# Dev/CI helper: fetch pinned, checksummed portable runtimes for the Linux sandbox
# attack suite and the judge tests: python-build-standalone and Node LTS. This is NOT
# the product download path (that lives in kb-toolchain and needs maintainer sign-off);
# it only feeds `cargo test -p kb-runner` and `cargo test -p kb-judge`.
#
# Also creates the dev sandbox root (target/katabox, marked with .katabox-sandbox-root),
# which mirrors the app layout: <root>/toolchains/<runtime> and <root>/runs/<run>.
# Linux counterpart of scripts/fetch-python-toolchain.ps1 (same releases, same layout).
#
# Usage: scripts/fetch-python-toolchain.sh [root]   (default: target/katabox)
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
root="${1:-$script_dir/../target/katabox}"
mkdir -p "$root/runs"
root="$(cd "$root" && pwd -P)"
[ -f "$root/.katabox-sandbox-root" ] || : > "$root/.katabox-sandbox-root"

# install_toolchain NAME URL SHA256 INNER CHECK DEST
# Download URL, verify SHA256, extract into an empty folder and move the archive's single
# top-level folder INNER to DEST. CHECK is an executable that must exist inside it.
install_toolchain() {
    local name="$1" url="$2" sha256="$3" inner="$4" check="$5" dest="$6"
    if [ -x "$dest/$check" ]; then
        echo "$name already present at $dest"
        return
    fi
    mkdir -p "$(dirname "$dest")"
    local work
    work="$(mktemp -d "$(dirname "$dest")/_fetch-XXXXXXXX")"
    # shellcheck disable=SC2064 # expand $work now: it is local to this call
    trap "rm -rf '$work'" RETURN EXIT

    local archive="$work/${url##*/}"
    echo "Downloading $name from $url"
    curl -fsSL --proto '=https' --tlsv1.2 -A katabox-dev -o "$archive" "$url"

    local actual
    actual="$(sha256sum "$archive" | cut -d' ' -f1)"
    if [ "$actual" != "$sha256" ]; then
        printf 'SHA-256 mismatch for %s\n expected %s\n actual   %s\n' "$name" "$sha256" "$actual" >&2
        exit 1
    fi
    echo "SHA-256 verified"

    # Extract into an empty folder; refuse absolute paths and `..` members.
    mkdir "$work/x"
    if tar -tf "$archive" | grep -qE '(^/|(^|/)\.\.(/|$))'; then
        echo "$name archive contains unsafe paths" >&2
        exit 1
    fi
    tar -xf "$archive" -C "$work/x" --no-same-owner --no-same-permissions

    [ -x "$work/x/$inner/$check" ] || { echo "$check not found in $name archive" >&2; exit 1; }
    rm -rf "$dest"
    mv "$work/x/$inner" "$dest"
    echo "$name installed at $dest"
}

# python-build-standalone. Checksums from the release's SHA256SUMS file.
py_release="20261026"
# Node.js LTS ("Lithium"). Checksums from the release's SHASUMS256.txt. Keep the version
# in sync with scripts/fetch-python-toolchain.ps1 and kb-judge's `toolchains/node-26`.
node_version="v26.13.0"
case "$(uname -m)" in
    x86_64)
        py_asset="cpython-3.14.10+$py_release-x86_64-unknown-linux-gnu-install_only.tar.gz"
        py_sha256="1b38024924eb7f4cf0cab45d7b871d1f1991354190fccd2625ac7529a43786e2"
        node_arch="x64"
        node_sha256="d28a96af6c38596f6d9d97c0ee531c13d238d757742387d1928de822d959e997"
        ;;
    aarch64)
        py_asset="cpython-3.14.10+$py_release-aarch64-unknown-linux-gnu-install_only.tar.gz"
        py_sha256="dcc31ce7b9ddbbffac160da7fff0cc069054f43eca52382a7d0eebf94e985b88"
        node_arch="arm64"
        node_sha256="406fe3fbde1c054adff8b297c16fdb5af6659db327c1ac8acd46b56ef1af9a92"
        ;;
    *)
        echo "unsupported architecture: $(uname -m)" >&2
        exit 1
        ;;
esac

install_toolchain "Python 3.14" \
    "https://github.com/astral-sh/python-build-standalone/releases/download/$py_release/${py_asset//+/%2B}" \
    "$py_sha256" python bin/python3 "$root/toolchains/python-3.14"

node_inner="node-$node_version-linux-$node_arch"
install_toolchain "Node $node_version" \
    "https://nodejs.org/dist/$node_version/$node_inner.tar.xz" \
    "$node_sha256" "$node_inner" bin/node "$root/toolchains/node-26"
