# V-OS War-Rig: CLI Build & Deploy Guide

This guide provides the exact Command Line Interface (CLI) commands to build the V-OS ISOs and flash them to USB.

## Option A: Build the Rust Kernel ISO
*For the custom bare-metal V-OS kernel.*

### 1. Build & Package
Run the PowerShell script from the project root:
```powershell
./build_v_os.ps1
```
*Output:* `V-OS-WarRig.iso`

---

## Option B: Build the Linux Installer ISO
*For the Arch Linux + BlackArch Zero-Touch Installer.*

### 1. Prepare Environment (Arch Linux Host)
```bash
sudo pacman -S archiso
mkdir -p ~/archlive
cp -r /usr/share/archiso/configs/releng/* ~/archlive/
```

### 2. Inject Custom Files
Assuming you are in the `v_os_war_rig` project root:

```bash
# Add Installer Script
sudo cp v_os_distro_build/unattended_install.sh ~/archlive/airootfs/root/
sudo chmod +x ~/archlive/airootfs/root/unattended_install.sh

# Add Auto-Run Hook
echo 'if [[ -z $DISPLAY ]] && [[ $(tty) = /dev/tty1 ]]; then ./unattended_install.sh; fi' | sudo tee -a ~/archlive/airootfs/root/.zlogin

# Create Data Directory
sudo mkdir -p ~/archlive/airootfs/opt/v_os

# Add AI Stub
sudo cp v_os_distro_build/ai_core.py ~/archlive/airootfs/opt/v_os/

# (Optional) Add Mistral 7B Weights for Offline Mode
# sudo cp /path/to/mistral_7b.gguf ~/archlive/airootfs/opt/v_os/
```

### 3. Build ISO
```bash
sudo mkarchiso -v -w /tmp/archiso-work -o out/ ~/archlive
```
*Output:* `out/archlinux-YYYY.MM.DD-x86_64.iso`

---

## Deploy to USB (Rufus)

**WARNING:** This will wipe all data on the USB drive.

### Step 1: Open Rufus
Download and run Rufus (Windows).

### Step 2: Configure
*   **Device:** Select your USB Drive.
*   **Boot Selection:** Browse and select your built `.iso` file (`V-OS-WarRig.iso` or the Arch Linux output).
*   **Partition Scheme:** `GPT`
*   **Target System:** `UEFI (non CSM)`
*   **File System:** `FAT32`

### Step 3: Flash
1.  Click **START**.
2.  **CRITICAL:** When prompted, select **Write in DD Image mode**.
    *   *Why?* "ISO Image mode" modifies the bootloader and breaks Arch Linux labels. "DD Image mode" writes the exact bit-for-bit copy required for the installer to work.
3.  Confirm data destruction warning.

### Step 4: Verification
Verify the USB structure and BIOS settings (Secure Boot DISABLED) before booting.
