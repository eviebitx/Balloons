import tkinter as tk
from tkinter import scrolledtext, messagebox
import subprocess
import threading
import sys
import os
import shutil

class VOsBuilderApp:
    def __init__(self, root):
        self.root = root
        self.root.title("V-OS War-Rig ISO Builder")
        self.root.geometry("600x400")

        # Header
        header = tk.Label(root, text="V-OS War-Rig Builder", font=("Helvetica", 16, "bold"))
        header.pack(pady=10)

        # Build Button
        self.build_btn = tk.Button(root, text="Build ISO", command=self.start_build, height=2, width=20, bg="#4CAF50", fg="white")
        self.build_btn.pack(pady=10)

        # Log Area
        self.log_area = scrolledtext.ScrolledText(root, state='disabled', height=15)
        self.log_area.pack(fill='both', expand=True, padx=10, pady=5)

        self.root.protocol("WM_DELETE_WINDOW", self.on_closing)

    def log(self, message):
        self.log_area.config(state='normal')
        self.log_area.insert(tk.END, message + "\n")
        self.log_area.see(tk.END)
        self.log_area.config(state='disabled')

    def start_build(self):
        self.build_btn.config(state='disabled')
        self.log("--- Starting Build Process ---")
        threading.Thread(target=self.run_build, daemon=True).start()

    def run_command(self, cmd, cwd=None):
        try:
            process = subprocess.Popen(
                cmd,
                cwd=cwd,
                stdout=subprocess.PIPE,
                stderr=subprocess.STDOUT,
                text=True,
                shell=True
            )

            for line in process.stdout:
                self.log(line.strip())

            process.wait()
            return process.returncode
        except Exception as e:
            self.log(f"Error executing command: {e}")
            return -1

    def run_build(self):
        # 1. Build Kernel
        self.log("[1/3] Compiling Rust Kernel...")
        # Use explicitly defined target to ensure we build for bare metal
        cmd_kernel = "cargo build --release --target x86_64-unknown-none"
        if self.run_command(cmd_kernel) != 0:
            self.log("[FAIL] Kernel compilation failed.")
            self.build_btn.config(state='normal')
            return

        # 2. Build Disk Image
        self.log("[2/3] Generating Bootable ISO (UEFI)...")
        kernel_bin = "target/x86_64-unknown-none/release/v_os_war_rig"

        # We need to run the 'builder' crate.
        # IMPORTANT: The builder crate itself runs on the HOST, so it must NOT use the x86_64-unknown-none target.
        # We must ensure .cargo/config.toml doesn't interfere, or override it.
        # But cargo run arguments are tricky.
        # Best way is to run it from the builder directory and ensure no global config forces target.
        # Or pass --target x86_64-unknown-linux-gnu (or whatever the host is).
        # We will try running without specific target flag, hoping it picks host default if not forced.

        # Check if kernel exists
        if not os.path.exists(kernel_bin):
             self.log(f"[FAIL] Kernel binary not found at {kernel_bin}")
             self.build_btn.config(state='normal')
             return

        # Run builder
        # Note: We pass the relative path to kernel bin from builder dir
        # MUST use nightly for bootloader crate
        cmd_image = f"cargo +nightly run --release --manifest-path builder/Cargo.toml -- ../{kernel_bin}"

        if self.run_command(cmd_image) != 0:
            self.log("[FAIL] Disk Image creation failed.")
            self.build_btn.config(state='normal')
            return

        # 3. Move Artifact
        self.log("[3/3] Finalizing Artifact...")
        src_iso = "builder/V-OS-WarRig.iso"
        dst_iso = "V-OS-WarRig.iso"

        if os.path.exists(src_iso):
            try:
                if os.path.exists(dst_iso):
                    os.remove(dst_iso)
                shutil.move(src_iso, dst_iso)
                self.log(f"[SUCCESS] ISO built successfully: {os.path.abspath(dst_iso)}")
                messagebox.showinfo("Success", f"ISO built successfully!\n{dst_iso}")
            except Exception as e:
                self.log(f"[FAIL] Failed to move ISO: {e}")
        else:
            self.log("[FAIL] Builder did not produce V-OS-WarRig.iso")

        self.build_btn.config(state='normal')

    def on_closing(self):
        self.root.destroy()

if __name__ == "__main__":
    root = tk.Tk()
    app = VOsBuilderApp(root)
    root.mainloop()
