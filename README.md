# V-OS Kernel

This is the source code for V-OS, a full-scale monolithic x86_64 OS with the integrated "V" personality.

## Building

Requires a cross-compiler (e.g., `x86_64-elf-g++`).

```bash
g++ -c src/kernel/*.cpp -I include -ffreestanding -mno-red-zone -fno-exceptions -fno-rtti
ld -T linker.ld -o kernel.bin src/kernel/*.o --oformat binary
```
