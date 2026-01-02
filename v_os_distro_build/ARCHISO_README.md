# V-OS War-Rig: ArchISO Customization Guide

This guide details how to bake the `unattended_install.sh` script into a custom Arch Linux ISO using `archiso`.

## 1. Setup ArchISO Environment

On an Arch Linux host (or container):
```bash
pacman -S archiso
mkdir -p ~/archlive
cp -r /usr/share/archiso/configs/releng/* ~/archlive/
```

## 2. Task 2: ArchISO Customization

### A. Add the Installer Script
Copy the `unattended_install.sh` script to the `airootfs` (Live OS root file system):

```bash
# Assuming you are in the v_os_distro_build directory
mkdir -p ~/archlive/airootfs/root/
cp unattended_install.sh ~/archlive/airootfs/root/
chmod +x ~/archlive/airootfs/root/unattended_install.sh
```

### B. Add AI Data (Offline Mode)
To support the "AI Provisioning" step of the installer and enable offline capabilities, place the data directly into the live root file system.

```bash
# Create a folder in the airootfs
mkdir -p ~/archlive/airootfs/opt/v_os
cp ai_core.py ~/archlive/airootfs/opt/v_os/
cp mistral_7b.gguf ~/archlive/airootfs/opt/v_os/
```
*Note: This increases the ISO size by the size of the model (~4GB for Mistral 7B).*

### C. Configure Auto-Execution via `.zlogin`
Modify `~/archlive/airootfs/root/.zlogin` to execute the installer automatically upon login.

**File:** `~/archlive/airootfs/root/.zlogin`
```bash
# ~/.zlogin
if [[ -z $DISPLAY ]] && [[ $(tty) = /dev/tty1 ]]; then
    echo "Welcome to V-OS War-Rig Live Environment."
    echo "Starting Unattended Installer..."
    ./unattended_install.sh
fi
```

### D. Configure Boot Timeout
Edit the bootloader configuration to reduce timeout.

**File:** `~/archlive/syslinux/syslinux.cfg` (BIOS) and `~/archlive/efiboot/loader/loader.conf` (UEFI).

For **UEFI** (`loader.conf`):
```text
timeout 1
default arch.conf
```

For **Syslinux**:
Change `TIMEOUT 50` (5 seconds) to `TIMEOUT 10` (1 second).

## 3. Task 3: The USB Ready Output

### Building the ISO
Run the following command to generate the ISO:

```bash
mkarchiso -v -w /tmp/archiso-work -o out/ ~/archlive
```

**Output:** `out/archlinux-YYYY.MM.DD-x86_64.iso` (Rename to `V-OS-WarRig-Install.iso`)

### Flashing with Rufus (DD Mode)

**Why "DD Mode"?**
The generated ISO is an "isohybrid" image.
- **ISO Mode:** Rufus attempts to extract files and create a FAT32 partition on the USB, installing its own GRUB/Syslinux. This often breaks Arch Linux boot labels and mounting paths (`/dev/disk/by-label/...`).
- **DD Mode (Disk Dump):** This writes the ISO bit-for-bit directly to the USB drive, preserving the exact partition structure, labels, and bootloaders created by `archiso`. This is **critical** for the live environment to correctly find its root file system and mount the AI data.

**Instructions:**
1. Select `V-OS-WarRig-Install.iso`.
2. Click **Start**.
3. When prompted, select **"Write in DD Image mode"**.
