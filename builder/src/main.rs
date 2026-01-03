use bootloader::DiskImageBuilder;
use std::{env, path::PathBuf};

fn main() {
    // Get the kernel binary path from arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: builder <kernel-executable-path>");
        std::process::exit(1);
    }
    let kernel_path = PathBuf::from(&args[1]);
    let out_path = PathBuf::from("V-OS-WarRig.iso"); // Using .iso extension for familiarity, though it's a UEFI disk image

    let mut builder = DiskImageBuilder::new(kernel_path);

    // Optional: Add ramdisks or extra files if needed (e.g. for AI weights if we were embedding them in the kernel image, but we are using distro ISO for that)
    // builder.set_ramdisk_image(PathBuf::from("ramdisk.img"));

    if let Err(e) = builder.create_uefi_image(&out_path) {
        eprintln!("Failed to create UEFI disk image: {}", e);
        std::process::exit(1);
    }

    println!("Successfully created UEFI Disk Image at: {}", out_path.display());
    println!("NOTE: This is a UEFI-only image. Use Rufus 'DD Mode' to flash.");
}
