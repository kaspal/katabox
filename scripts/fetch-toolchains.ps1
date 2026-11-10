# Dev/CI helper: fetch pinned, checksummed portable runtimes for the Windows sandbox
# attack suite and the judge end-to-end tests: python-build-standalone, Node LTS, PHP and
# Rust (windows-gnu).
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
# single top-level folder $Inner (or everything, if $Inner is empty) to $Dest. $Check is
# a file that must exist inside it.
function Install-Toolchain {
    param([string]$Name, [string]$Url, [string]$Sha256, [AllowEmptyString()][string]$Inner, [string]$Check, [string]$Dest)

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

        # An empty $Inner means the archive has no top-level folder (PHP's zip).
        $Top = if ($Inner) { Join-Path $Extract $Inner } else { $Extract }
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

# PHP 8.5 NTS x64 (VS17 build). Checksum from windows.php.net/downloads/releases/releases.json.
# php.exe and php8.dll need only Windows system DLLs plus VCRUNTIME140.dll (ADR 0009).
$PhpVersion = "8.5.12-1"
$PhpDest = Join-Path $Root "toolchains\php-8.5"
Install-Toolchain -Name "PHP $PhpVersion" `
    -Url "https://windows.php.net/downloads/releases/php-$PhpVersion-nts-Win32-vs17-x64.zip" `
    -Sha256 "2a36a2edcb7a5b9252f638fd5e4cf84d7f6e066092654220dc0b63e5cf4e9723" `
    -Inner "" -Check "php.exe" `
    -Dest $PhpDest

# App-local VC++ runtime for PHP, so users never install a redistributable. Copied from the
# pinned Python build above (Microsoft-signed, 14.44.35211); PHP 8.5.12 is linked with the
# 14.44 toolset and needs a runtime of at least that version.
$VcRuntime = Join-Path $PhpDest "vcruntime140.dll"
$VcSha256 = "d5e4d9a3e835fa679450145d6a7d94e36573a509317111904d9b3712c30d9066"
if (-not (Test-Path $VcRuntime) -or (Get-FileHash -Algorithm SHA256 $VcRuntime).Hash.ToLowerInvariant() -ne $VcSha256) {
    $VcSource = Join-Path $Root "toolchains\python-3.14\vcruntime140.dll"
    $Actual = (Get-FileHash -Algorithm SHA256 $VcSource).Hash.ToLowerInvariant()
    if ($Actual -ne $VcSha256) {
        throw "SHA-256 mismatch for $VcSource`n expected $VcSha256`n actual   $Actual"
    }
    $Version = [version](Get-Item $VcSource).VersionInfo.FileVersion
    if ($Version -lt [version]"14.44.0.0") { throw "vcruntime140.dll $Version is older than 14.44" }
    Copy-Item $VcSource $VcRuntime -Force
    Write-Host "vcruntime140.dll $Version copied next to php.exe"
}

# Rust for the x86_64-pc-windows-gnu host: rustc, rust-std and rust-mingw (the
# self-contained linker, so users never need Visual Studio), merged into one sysroot.
# Checksums from dist/<date>/channel-rust-stable.toml. Keep in sync with kb-judge's
# dev toolchain path (`toolchains/rust-1.99`).
function Install-RustComponents {
    param([string]$Version, [string]$Date, [hashtable]$Components, [string]$Dest)

    if (Test-Path (Join-Path $Dest "bin\rustc.exe")) {
        Write-Host "Rust $Version already present at $Dest"
        return
    }
    $Parent = Split-Path $Dest -Parent
    New-Item -ItemType Directory -Force $Parent | Out-Null
    $Work = Join-Path $Parent ("_fetch-" + [guid]::NewGuid().ToString("N"))
    $Stage = Join-Path $Work "sysroot"
    New-Item -ItemType Directory -Force $Stage | Out-Null
    try {
        foreach ($Component in $Components.Keys) {
            $Base = "$Component-$Version-x86_64-pc-windows-gnu"
            $Url = "https://static.rust-lang.org/dist/$Date/$Base.tar.xz"
            $Archive = Join-Path $Work "$Base.tar.xz"
            Write-Host "Downloading $Base"
            Invoke-WebRequest -Uri $Url -OutFile $Archive -Headers @{ "User-Agent" = "katabox-dev" }
            $Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
            if ($Actual -ne $Components[$Component]) {
                throw "SHA-256 mismatch for $Base`n expected $($Components[$Component])`n actual   $Actual"
            }
            $Extract = Join-Path $Work $Component
            New-Item -ItemType Directory $Extract | Out-Null
            & "$env:SystemRoot\System32\tar.exe" -xf $Archive -C $Extract
            if ($LASTEXITCODE -ne 0) { throw "tar failed with exit code $LASTEXITCODE" }
            # Each tarball holds <base>/<component-dir>/{bin,lib}; merge those into the sysroot.
            Get-ChildItem (Join-Path $Extract $Base) -Directory | ForEach-Object {
                Copy-Item (Join-Path $_.FullName "*") $Stage -Recurse -Force
            }
            Remove-Item $Archive
        }
        if (-not (Test-Path (Join-Path $Stage "bin\rustc.exe"))) { throw "rustc.exe missing after merge" }
        if (Test-Path $Dest) { Remove-Item -Recurse -Force $Dest }
        Move-Item $Stage $Dest
        Write-Host "Rust $Version installed at $Dest"
    }
    finally {
        Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
    }
}

Install-RustComponents -Version "1.99.1" -Date "2026-10-15" -Dest (Join-Path $Root "toolchains\rust-1.99") -Components @{
    "rustc"      = "a48f186defb3d57538b34e14a24a94f6e29906f191630b627dcb091198bae438"
    "rust-std"   = "b17e3bc962ff24b3bc47a750d838defed85b280746250e3a3ad029e6c1a29dd7"
    "rust-mingw" = "1dca8e0d253565dbf21296eabc3e8d6e06e607e92c59dc00e3ecff902fe3d5ae"
}
