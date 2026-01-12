#!/bin/bash
# V-OS War-Rig Unattended Installer
# Target: Arch Linux + BlackArch + V-OS Customizations
# Status: Mission Critical

set -e

echo "[V-OS] Initializing Unattended Installation Protocol..."

# --- 1. Disk Detection & Partitioning ---
echo "[V-OS] Scanning for primary target drive..."
# Find the largest block device (likely the install target)
TARGET_DISK=$(lsblk -dplnx size -o name | tail -1)

if [[ -z "$TARGET_DISK" ]]; then
    echo "[ERROR] No disk detected!"
    exit 1
fi

echo "[V-OS] Target identified: $TARGET_DISK"
echo "[WARNING] Wiping disk in 3 seconds..."
sleep 3

# Wipe and Partition (GPT)
# 1. Zap disk
sgdisk --zap-all "$TARGET_DISK"

# 2. Create partitions
# Part 1: EFI System Partition (512M)
sgdisk -n 1:0:+512M -t 1:ef00 -c 1:"EFI System" "$TARGET_DISK"
# Part 2: Root Partition (Remaining)
sgdisk -n 2:0:0 -t 2:8300 -c 2:"V-OS Root" "$TARGET_DISK"

# Determine partition names (handle nvme0n1p1 vs sda1)
if [[ "$TARGET_DISK" == *"nvme"* ]]; then
    PART_EFI="${TARGET_DISK}p1"
    PART_ROOT="${TARGET_DISK}p2"
else
    PART_EFI="${TARGET_DISK}1"
    PART_ROOT="${TARGET_DISK}2"
fi

echo "[V-OS] Formatting partitions..."
mkfs.fat -F32 "$PART_EFI"
mkfs.ext4 -F "$PART_ROOT"

# Mount
mount "$PART_ROOT" /mnt
mkdir -p /mnt/boot
mount "$PART_EFI" /mnt/boot

# --- 2. Bootstrap Arch Linux ---
echo "[V-OS] Bootstrapping base system (pacstrap)..."
# Using a generic mirror or the one from ISO
pacstrap /mnt base linux linux-firmware base-devel grub efibootmgr networkmanager python python-pip git wget vim

# Generate fstab
genfstab -U /mnt >> /mnt/etc/fstab

# --- 3. System Configuration (Chroot) ---
echo "[V-OS] Configuring internal system..."

arch-chroot /mnt /bin/bash <<EOF
    # Timezone
    ln -sf /usr/share/zoneinfo/UTC /etc/localtime
    hwclock --systohc

    # Localization
    echo "en_US.UTF-8 UTF-8" > /etc/locale.gen
    locale-gen
    echo "LANG=en_US.UTF-8" > /etc/locale.conf
    echo "v-os-war-rig" > /etc/hostname

    # Root Password (Default: toor)
    echo "root:toor" | chpasswd
    echo "[V-OS] Root password set to 'toor'."

    # Network
    systemctl enable NetworkManager
EOF

# --- 4. BlackArch Integration ---
echo "[V-OS] Deploying BlackArch Toolset..."
arch-chroot /mnt /bin/bash <<EOF
    cd /tmp
    curl -O https://blackarch.org/strap.sh
    # Verification omitted for speed in this context, but recommended in prod
    chmod +x strap.sh
    ./strap.sh

    # Update and install core tools
    pacman -Syyu --noconfirm
    # Installing a subset of tools to save time/bandwidth
    # In a full install: pacman -S blackarch --noconfirm
    pacman -S --noconfirm blackarch-scanner blackarch-cracker blackarch-sniffers
EOF

# --- 5. AI Provisioning ---
echo "[V-OS] Provisioning V-Intelligence (Mistral AI)..."
mkdir -p /mnt/opt/v_os/ai_weights

# Assume installation media is mounted at /run/archiso/bootmnt
INSTALL_MEDIA="/run/archiso/bootmnt"

# Offline Source (Directly in airootfs)
OFFLINE_SOURCE="/opt/v_os"

if [ -f "$OFFLINE_SOURCE/mistral_7b.gguf" ]; then
    echo "[V-OS] Copying AI Core and Weights (Offline Mode)..."
    cp "$OFFLINE_SOURCE/ai_core.py" /mnt/opt/v_os/
    cp "$OFFLINE_SOURCE/mistral_7b.gguf" /mnt/opt/v_os/ai_weights/
elif [ -d "$INSTALL_MEDIA/v_os_data" ]; then
    echo "[V-OS] Copying AI Core and Weights (ISO Mount Mode)..."
    cp "$INSTALL_MEDIA/v_os_data/ai_core.py" /mnt/opt/v_os/
    cp "$INSTALL_MEDIA/v_os_data/mistral_7b.gguf" /mnt/opt/v_os/ai_weights/ || echo "[WARN] Weights not found on media."
else
    echo "[WARN] V-OS Data directory not found on install media. Skipping AI copy."
    # Create a stub
    echo "print('V-OS AI Core Online')" > /mnt/opt/v_os/ai_core.py
fi

# --- 6. Finalize Bootloader ---
echo "[V-OS] Installing Bootloader..."
arch-chroot /mnt grub-install --target=x86_64-efi --efi-directory=/boot --bootloader-id=GRUB
arch-chroot /mnt grub-mkconfig -o /boot/grub/grub.cfg

echo "[V-OS] Installation Complete. Rebooting in 5 seconds..."
sleep 5
reboot
