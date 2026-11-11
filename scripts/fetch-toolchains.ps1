# Dev/CI helper: fetch pinned, checksummed portable runtimes for the Windows sandbox
# attack suite and the judge end-to-end tests: python-build-standalone, Node LTS, PHP and
# Rust (windows-gnu).
# This is NOT the product download path (that lives in kb-toolchain and needs maintainer
# sign-off); it only feeds `cargo test -p kb-runner` and `cargo test -p kb-judge`.
#
# Also creates the dev sandbox root (target/katabox, marked with .katabox-sandbox-root),
# which mirrors the app layout: <root>/toolchains/<runtime> and <root>/runs/<run>.
#
# Also fetches the language servers (ADR 0014) into <root>/tools unless -NoLanguageServers.
#
# Usage: pwsh scripts/fetch-toolchains.ps1 [-Root target/katabox] [-NoLanguageServers]

param(
    [string]$Root = (Join-Path $PSScriptRoot "..\target\katabox"),
    [switch]$NoLanguageServers
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

# --- Language servers (ADR 0014) ------------------------------------------------------
# They run outside the judge sandbox and never execute user code; see the ADR for the
# settings that guarantee it. Everything is pinned and verified here.
if ($NoLanguageServers) {
    Write-Host "Skipping language servers (-NoLanguageServers)"
    return
}
$Tools = Join-Path $Root "tools"
New-Item -ItemType Directory -Force $Tools | Out-Null

# rust-src: std's sources, so rust-analyzer can resolve std (merged into the sysroot).
$RustSysroot = Join-Path $Root "toolchains\rust-1.99"
$RustSrcCheck = Join-Path $RustSysroot "lib\rustlib\src\rust\library\std\src\lib.rs"
if (-not (Test-Path $RustSrcCheck)) {
    $Work = Join-Path $Tools ("_fetch-" + [guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Force $Work | Out-Null
    try {
        $Base = "rust-src-1.99.1"
        $Archive = Join-Path $Work "$Base.tar.xz"
        Write-Host "Downloading $Base"
        Invoke-WebRequest -Uri "https://static.rust-lang.org/dist/2026-10-15/$Base.tar.xz" -OutFile $Archive -Headers @{ "User-Agent" = "katabox-dev" }
        $Actual = (Get-FileHash -Algorithm SHA256 $Archive).Hash.ToLowerInvariant()
        $Expected = "e485a88c45db546b73571df7bebfcb4454ba4fb552c492e25fc5b00eeb5bf22d"
        if ($Actual -ne $Expected) { throw "SHA-256 mismatch for $Base`n expected $Expected`n actual   $Actual" }
        & "$env:SystemRoot\System32\tar.exe" -xf $Archive -C $Work
        if ($LASTEXITCODE -ne 0) { throw "tar failed with exit code $LASTEXITCODE" }
        Copy-Item (Join-Path $Work "$Base\rust-src\*") $RustSysroot -Recurse -Force
        if (-not (Test-Path $RustSrcCheck)) { throw "rust-src did not install std sources" }
        Write-Host "rust-src installed into $RustSysroot"
    }
    finally {
        Remove-Item -Recurse -Force $Work -ErrorAction SilentlyContinue
    }
}

# rust-analyzer matching the pinned Rust (channel-rust-stable.toml, rust-analyzer-preview).
Install-Toolchain -Name "rust-analyzer 1.99.1" `
    -Url "https://static.rust-lang.org/dist/2026-10-15/rust-analyzer-1.99.1-x86_64-pc-windows-gnu.tar.xz" `
    -Sha256 "15542a6a2a264f1b3daf7141640690d8ae5a594719ff283ff1750358fdbbc8f1" `
    -Inner "rust-analyzer-1.99.1-x86_64-pc-windows-gnu\rust-analyzer-preview" -Check "bin\rust-analyzer.exe" `
    -Dest (Join-Path $Tools "rust-analyzer")

# phpactor is not fetched on Windows: it requires the PHP posix extension, which Windows
# builds of PHP do not have (ADR 0014). Pinned for Linux: 2026.10.07.0, phpactor.phar
# SHA-256 b28bf6166e016d539fc2f2aec18ef918b12590e436a6f1ccbdf30b5bbdbdb16b.

# basedpyright, typescript-language-server and typescript from the committed lockfile
# (scripts/language-servers/package-lock.json, integrity hashes checked by npm), with the
# bundled Node's npm. No install scripts run.
$LsSource = Join-Path $PSScriptRoot "language-servers"
$LsDest = Join-Path $Tools "language-servers"
$LockHash = (Get-FileHash -Algorithm SHA256 (Join-Path $LsSource "package-lock.json")).Hash
$Stamp = Join-Path $LsDest ".lock-sha256"
if (-not (Test-Path $Stamp) -or (Get-Content $Stamp -Raw).Trim() -ne $LockHash) {
    if (Test-Path $LsDest) { Remove-Item -Recurse -Force $LsDest }
    New-Item -ItemType Directory -Force $LsDest | Out-Null
    Copy-Item (Join-Path $LsSource "package.json"), (Join-Path $LsSource "package-lock.json") $LsDest
    $NodeDir = Join-Path $Root "toolchains\node-26"
    $env:Path = "$NodeDir;$env:Path"
    Push-Location $LsDest
    try {
        & (Join-Path $NodeDir "npm.cmd") ci --ignore-scripts --no-audit --no-fund --omit=optional
        if ($LASTEXITCODE -ne 0) { throw "npm ci failed with exit code $LASTEXITCODE" }
    }
    finally {
        Pop-Location
    }
    foreach ($Check in "node_modules\basedpyright\langserver.index.js", "node_modules\typescript-language-server\lib\cli.mjs", "node_modules\typescript\lib\tsserver.js") {
        if (-not (Test-Path (Join-Path $LsDest $Check))) { throw "$Check missing after npm ci" }
    }
    Set-Content -Path $Stamp -Value $LockHash
    Write-Host "Node language servers installed at $LsDest"
}

