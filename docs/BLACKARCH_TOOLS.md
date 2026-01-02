# BlackArch Toolset Request

The user has requested the BlackArch Linux toolset to be available at install.

Since this is a kernel source repository ("V-OS"), the toolset installation will be part of the user-space distribution build process (distro construction).

## Implementation Note

When building the root filesystem (rootfs) for the OS, the package manager (e.g., `pacman` if based on Arch/BlackArch) should be configured to include the `blackarch` repository and install the requested tools.

This file serves as a placeholder and acknowledgement of the requirement for the future OS build system.
