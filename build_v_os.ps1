# V-OS Build & ISO Packaging Script
Write-Host "--- V-OS War-Rig: Initiating Build Sequence ---" -ForegroundColor Cyan

# 1. Compile the Rust Kernel
Write-Host "[1/2] Compiling Rust Kernel..." -ForegroundColor Yellow
cargo build --release --target x86_64-unknown-none
if ($LASTEXITCODE -ne 0) { Write-Error "Compilation Failed!"; exit }

# 2. Build Disk Image using Helper
Write-Host "[2/2] Generating Bootable ISO (UEFI)..." -ForegroundColor Yellow
$kernelBin = "../target/x86_64-unknown-none/release/v_os_war_rig"

Set-Location builder
cargo +nightly run --release -- $kernelBin
if ($LASTEXITCODE -ne 0) { Write-Error "Disk Image Creation Failed!"; Set-Location ..; exit }
Set-Location ..

if (Test-Path "builder/V-OS-WarRig.iso") {
    Move-Item "builder/V-OS-WarRig.iso" -Destination "." -Force
    Write-Host "--- SUCCESS: V-OS-WarRig.iso is ready! ---" -ForegroundColor Green
} else {
    Write-Error "ISO not found after build!"
}
