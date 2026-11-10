# Dev/CI helper: fetch pinned, checksummed portable runtimes for the Windows sandbox
# attack suite and the judge end-to-end tests: python-build-standalone and Node LTS.
# This is NOT the product download path (that lives in kb-toolchain and needs maintainer
# sign-off); it only feeds `cargo test -p kb-runner` and `cargo test -p kb-judge`.
#
# Also creates the dev sandbox root (target/katabox, marked with .katabox-sandbox-root),
# which mirrors the app layout: <root>/toolchains/<runtime> and <root>/runs/<run>.
#
# Usage: pwsh scripts/fetch-toolchains.ps1 [-Root target/katabox]

param(
    [string]$Root = (Join-Path $PSScriptRoot "..\target\katabox")
)

$ErrorActionPreference = "Stop"

$Root = [System.IO.Path]::GetFullPath($Root)
New-Item -ItemType Directory -Force (Join-Path $Root "runs") | Out-Null
$Marker = Join-Path $Root ".katabox-sandbox-root"
if (-not (Test-Path $Marker)) { New-Item -ItemType File $Marker | Out-Null }

# Download $Url, verify $Sha256, extract it into an empty folder and move the archive's
# single top-level folder $Inner to $Dest. $Check is a file that must exist inside it.
function Install-Toolchain {
    param([string]$Name, [string]$Url, [string]$Sha256, [string]$Inner, [string]$Check, [string]$Dest)

    if (Test-Path (Join-Path $Dest $Check)) {
        Write-Host "$Name already present at $Dest"
        return
    }
    $Parent = Split-Path $Dest -Parent
    New-Item -ItemType Directory -Force $Parent | Out-Null
    $Work = Join-Path $Parent ("_fetch-" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force $Work | Out-Null
    try {
        $Archive = Join-Path $Work ([System.IO.Path]::GetFileName(([uri]$Url).LocalPath))
        Write-Host "Downloading $Name from $Url"
        Invoke-WebRequest -Uri $Url -OutFile $Archive -Headers @{ "User-Agent" = "katabox-dev" }

        $Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
        if ($Actual -ne $Sha256) {
            throw "SHA-256 mismatch for $Name`n expected $Sha256`n actual   $Actual"
        }
        Write-Host "SHA-256 verified"

        # bsdtar reads both .tar.gz and .zip and refuses absolute and `..` members.
        $Extract = Join-Path $Work "x"
        New-Item -ItemType Directory $Extract | Out-Null
        & "$env:SystemRoot\System32\tar.exe" -xf $Archive -C $Extract
        if ($LASTEXITCODE -ne 0) { throw "tar failed with exit code $LASTEXITCODE" }

        $Top = Join-Path $Extract $Inner
        if (-not (Test-Path (Join-Path $Top $Check))) { throw "$Check not found in $Name archive" }
        if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }
        Move-Item $Top $Dest
        Write-Host "$Name installed at $Dest"
    }
    finally {
        Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
    }
}

# python-build-standalone. Checksum from the release's SHA256SUMS file.
$PyRelease = "20261026"
$PyAsset = "cpython-3.14.10+$PyRelease-x86_64-pc-windows-msvc-install_only.tar.gz"
Install-Toolchain -Name "Python 3.14" `
    -Url ("https://github.com/astral-sh/python-build-standalone/releases/download/$PyRelease/" + [uri]::EscapeDataString($PyAsset)) `
    -Sha256 "98bbe6a0d798bcbad142e96e5930a330e4dc407638e898b515c51eb73a27580d" `
    -Inner "python" -Check "python.exe" `
    -Dest (Join-Path $Root "toolchains\python-3.14")

# Node.js LTS ("Lithium"), portable zip. Checksum from the release's SHASUMS256.txt.
# Keep the version in sync with scripts/fetch-toolchains.sh and kb-judge's
# dev toolchain path (`toolchains/node-26`).
$NodeVersion = "v26.13.0"
Install-Toolchain -Name "Node $NodeVersion" `
    -Url "https://nodejs.org/dist/$NodeVersion/node-$NodeVersion-win-x64.zip" `
    -Sha256 "dcc60d59a0746999f99064a01c020445d54d84228c691cdbf25b49ac3b59150a" `
    -Inner "node-$NodeVersion-win-x64" -Check "node.exe" `
    -Dest (Join-Path $Root "toolchains\node-26")
