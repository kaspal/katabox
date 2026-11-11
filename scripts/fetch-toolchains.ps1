# Dev/CI helper: install every language (runtimes and language servers) into the dev
# sandbox root, through the same code the app uses (kb-toolchain, ADR 0015). The pins,
# sizes and SHA-256 hashes live only in crates/kb-toolchain/manifest.toml.
#
# Usage: pwsh scripts/fetch-toolchains.ps1 [-Root target/katabox] [-NoLanguageServers]
#
# A root made by an older version of this script has no install receipts: it is cleaned
# and installed again once.

param(
    [string]$Root = (Join-Path $PSScriptRoot "..\target\katabox"),
    [switch]$NoLanguageServers
)

$ErrorActionPreference = "Stop"
$Root = [System.IO.Path]::GetFullPath($Root)

$Args = @("run", "-q", "-p", "kb-toolchain", "--release", "--", "install", "--root", $Root, "--create-root", "--all")
if ($NoLanguageServers) { $Args += "--runtimes-only" }

Push-Location (Join-Path $PSScriptRoot "..")
try {
    & cargo @Args
    if ($LASTEXITCODE -ne 0) { throw "kb-toolchain install failed with exit code $LASTEXITCODE" }
}
finally {
    Pop-Location
}
