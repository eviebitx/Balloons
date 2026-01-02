# V-OS War-Rig

**Mission Status:** Active
**Codename:** War-Rig
**Kernel Type:** Monolithic, Rust-based (x86_64)

The V-OS War-Rig is a specialized, bare-metal operating system kernel designed for offensive security operations and high-performance hardware control. It features a custom "V" personality interface, integrated BlackArch tool support, and a split-screen graphical environment.

## System Architecture

- **Kernel:** Rust `no_std` implementation.
- **Graphics:** UEFI-compatible V-Graphics Engine with split-screen Window Manager.
  - **Left Console:** Mission Control & AI Analysis.
  - **Right Console:** Interactive Shell & Tool Output.
- **Input:** Polling-based PS/2 Keyboard Driver.
- **AI Integration:** "Mistrial 123b" Neural Interface stub.
- **Drivers:**
  - PCI Bus Scanner (Port I/O 0xCF8/0xCFC).
  - UART 16550 Serial Driver (COM1).
  - V-Console (VGA/Serial Hybrid).

## Build Instructions

### Prerequisites
- Rust Nightly (or compatible Stable with `bootloader_api` 0.11).
- `cargo bootimage` or `bootloader` build tools.
- `xorriso` (for ISO creation).

### Building
Run the provided PowerShell build script:
```powershell
./build_v_os.ps1
```
This will compile the kernel, create a bootable image, and package it into `V-OS-WarRig.iso`.

---

## Physical Hardware Installation

To deploy the V-OS War-Rig to physical hardware via USB, follow these precise steps using **Rufus**.

### Rufus Settings
1.  **Device:** Select your target USB Drive (Warning: All data will be destroyed).
2.  **Boot Selection:** Select `V-OS-WarRig.iso`.
3.  **Partition Scheme:** **GPT** (Essential for UEFI).
4.  **Target System:** **UEFI (non CSM)**.
5.  **File System:** **FAT32** (Default).
6.  **Cluster Size:** Default.

Click **START** and wait for the process to complete.

### Hardware Compatibility & BIOS Settings
**CRITICAL:** Before booting, you must configure your target machine's BIOS/UEFI settings:

*   **Secure Boot:** **DISABLED**.
    *   *Reason:* The V-OS kernel is a custom build and is not digitally signed by Microsoft or a trusted CA. Secure Boot will reject the bootloader if not disabled.
*   **Boot Mode:** **UEFI Only** (Legacy/CSM should be disabled if possible, though Hybrid might work).
*   **Fast Boot:** **Disabled** (Recommended to ensure hardware detection).

## Troubleshooting

### Black Screen / No Video
If the graphical interface fails to load or the screen remains black:

1.  **Serial Console (RS-232):**
    *   The War-Rig broadcasts all kernel logs and panic messages to **COM1 (0x3F8)**.
    *   Connect a null-modem cable to the physical serial port, or use a **USB-to-TTL Serial Adapter** connected to the motherboard's serial header.
    *   **Baud Rate:** 115200 (Standard).
    *   Monitor the output using `putty`, `minicom`, or `screen`.

2.  **Verify UEFI:**
    *   Ensure you are booting in UEFI mode. The graphics driver relies on the UEFI Framebuffer protocol (GOP). Legacy BIOS boot will result in no graphics.

---

*"Boss, give me a target and let's tear it apart."*
