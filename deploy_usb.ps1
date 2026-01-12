# V-OS Deployment Checklist Script
Write-Host "--- V-OS War-Rig: Deployment Pre-Flight Check ---" -ForegroundColor Cyan

$isoPath = "V-OS-WarRig.iso"

# 1. Verify ISO Existence
if (Test-Path $isoPath) {
    Write-Host "[OK] ISO Image Found: $isoPath" -ForegroundColor Green
} else {
    Write-Error "[FAIL] ISO Image NOT Found! Please run build_v_os.ps1 first."
    exit
}

# 2. Checklist
Write-Host "`n--- Deployment Checklist ---" -ForegroundColor Yellow
Write-Host "[ ] Target USB Drive plugged in?"
Write-Host "[ ] Data on USB backed up? (It will be wiped)"
Write-Host "[ ] Rufus installed/downloaded?"

Write-Host "`n--- BIOS/UEFI Settings Reminder ---" -ForegroundColor Red
Write-Host "1. Secure Boot: MUST BE DISABLED"
Write-Host "2. Boot Mode: UEFI (non-CSM)"

Write-Host "`n--- Rufus Configuration ---" -ForegroundColor Cyan
Write-Host "Partition Scheme: GPT"
Write-Host "Target System: UEFI (non CSM)"
Write-Host "File System: FAT32"

$confirmation = Read-Host "`nAre you ready to proceed to Rufus? (y/n)"
if ($confirmation -eq 'y') {
    Write-Host "Launching instructions... Good luck, Boss." -ForegroundColor Green
    # Ideally we'd launch Rufus here if we knew the path, but we'll just exit successfully.
} else {
    Write-Host "Deployment aborted." -ForegroundColor Yellow
}
