# Dev/CI helper: fetch a pinned, checksummed python-build-standalone build for the
# Windows sandbox attack suite. This is NOT the product download path (that lives in
# kb-toolchain and needs maintainer sign-off); it only feeds `cargo test -p kb-runner`.
#
# Usage: pwsh scripts/fetch-python-toolchain.ps1 [-Dest target/toolchains/python-3.14]

param(
    [string]$Dest = (Join-Path $PSScriptRoot "..\target\toolchains\python-3.14")
)

$ErrorActionPreference = "Stop"

$Release = "20261026"
$Asset = "cpython-3.14.10+$Release-x86_64-pc-windows-msvc-install_only.tar.gz"
# From the release's SHA256SUMS file.
$Sha256 = "98bbe6a0d798bcbad142e96e5930a330e4dc407638e898b515c51eb73a27580d"
$Url = "https://github.com/astral-sh/python-build-standalone/releases/download/$Release/" + [uri]::EscapeDataString($Asset)

$Dest = [System.IO.Path]::GetFullPath($Dest)
if (Test-Path (Join-Path $Dest "python.exe")) {
    Write-Host "Toolchain already present at $Dest"
    exit 0
}

$Parent = Split-Path $Dest -Parent
$Work = Join-Path $Parent ("_fetch-" + [guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force $Work | Out-Null
try {
    $Archive = Join-Path $Work "python.tar.gz"
    Write-Host "Downloading $Asset"
    Invoke-WebRequest -Uri $Url -OutFile $Archive -Headers @{ "User-Agent" = "katabox-dev" }

    $Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
    if ($Actual -ne $Sha256) {
        throw "SHA-256 mismatch for $Asset`n expected $Sha256`n actual   $Actual"
    }
    Write-Host "SHA-256 verified"

    $Extract = Join-Path $Work "x"
    New-Item -ItemType Directory $Extract | Out-Null
    & "$env:SystemRoot\System32\tar.exe" -xzf $Archive -C $Extract
    if ($LASTEXITCODE -ne 0) { throw "tar failed with exit code $LASTEXITCODE" }

    # The install_only archive has a single top-level `python/` directory.
    $Root = Join-Path $Extract "python"
    if (-not (Test-Path (Join-Path $Root "python.exe"))) { throw "python.exe not found in archive" }
    if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }
    Move-Item $Root $Dest
    Write-Host "Toolchain installed at $Dest"
}
finally {
    Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
}
