# V-OS Build & ISO Packaging Script
Write-Host "--- V-OS War-Rig: Initiating Build Sequence ---" -ForegroundColor Cyan

# 1. Compile the Rust Kernel
Write-Host "[1/3] Compiling Rust Kernel (Nightly)..." -ForegroundColor Yellow
cargo build --release
if ($LASTEXITCODE -ne 0) { Write-Error "Compilation Failed!"; exit }

# 2. Create the Bootable Binary
Write-Host "[2/3] Generating Boot Image..." -ForegroundColor Yellow
cargo bootimage --release
if ($LASTEXITCODE -ne 0) { Write-Error "Bootimage Generation Failed!"; exit }

# 3. Create the ISO (Assuming xorriso is installed via Choco or WSL)
# Note: This step packages the bin into a format Vultr can boot.
Write-Host "[3/3] Packaging into V-OS-WarRig.iso..." -ForegroundColor Yellow
$binPath = "target/x86_64-unknown-none/release/bootimage-v_os_war_rig.bin"
$isoPath = "V-OS-WarRig.iso"

# Using xorriso to create a hybrid ISO
xorriso -as mkisofs -R -f -e $binPath -no-emul-boot -o $isoPath .
if ($LASTEXITCODE -ne 0) { Write-Error "ISO Creation Failed!"; exit }

Write-Host "--- SUCCESS: V-OS-WarRig.iso is ready for deployment! ---" -ForegroundColor Green
