import tkinter as tk
from tkinter import filedialog, messagebox, scrolledtext
import zipfile
import os
import sys
import threading
import shutil
import tempfile
try:
    import pycdlib
except ImportError:
    messagebox.showerror("Error", "Required library 'pycdlib' not found.\nPlease run: pip install pycdlib")
    sys.exit(1)

class ZipToIsoConverter:
    def __init__(self, root):
        self.root = root
        self.root.title("ZIP to ISO Converter")
        self.root.geometry("600x450")

        # Variables
        self.zip_path = tk.StringVar()
        self.iso_path = tk.StringVar()

        # UI Components
        self.create_widgets()

    def create_widgets(self):
        # Header
        header = tk.Label(self.root, text="Zip to Bootable ISO", font=("Arial", 16, "bold"))
        header.pack(pady=10)

        # Source Selection
        frame_src = tk.Frame(self.root)
        frame_src.pack(fill='x', padx=20, pady=5)

        tk.Label(frame_src, text="Source Zip File:").pack(anchor='w')
        entry_src = tk.Entry(frame_src, textvariable=self.zip_path, width=50)
        entry_src.pack(side='left', fill='x', expand=True, padx=(0, 5))
        btn_src = tk.Button(frame_src, text="Browse...", command=self.select_zip)
        btn_src.pack(side='right')

        # Destination Selection
        frame_dst = tk.Frame(self.root)
        frame_dst.pack(fill='x', padx=20, pady=5)

        tk.Label(frame_dst, text="Destination ISO File:").pack(anchor='w')
        entry_dst = tk.Entry(frame_dst, textvariable=self.iso_path, width=50)
        entry_dst.pack(side='left', fill='x', expand=True, padx=(0, 5))
        btn_dst = tk.Button(frame_dst, text="Save As...", command=self.save_iso)
        btn_dst.pack(side='right')

        # Convert Button
        self.btn_convert = tk.Button(self.root, text="Convert to ISO", command=self.start_conversion,
                                     bg="#2196F3", fg="white", font=("Arial", 12, "bold"), height=2)
        self.btn_convert.pack(pady=20, fill='x', padx=100)

        # Log Area
        tk.Label(self.root, text="Status Log:").pack(anchor='w', padx=20)
        self.log_area = scrolledtext.ScrolledText(self.root, height=10, state='disabled')
        self.log_area.pack(fill='both', expand=True, padx=20, pady=(0, 20))

    def log(self, message):
        self.log_area.config(state='normal')
        self.log_area.insert(tk.END, message + "\n")
        self.log_area.see(tk.END)
        self.log_area.config(state='disabled')

    def select_zip(self):
        filename = filedialog.askopenfilename(title="Select Zip File",
                                              filetypes=[("Zip Files", "*.zip"), ("All Files", "*.*")])
        if filename:
            self.zip_path.set(filename)
            # Auto-suggest ISO name
            base = os.path.splitext(filename)[0]
            self.iso_path.set(base + ".iso")

    def save_iso(self):
        filename = filedialog.asksaveasfilename(title="Save ISO As",
                                                defaultextension=".iso",
                                                filetypes=[("ISO Files", "*.iso")])
        if filename:
            self.iso_path.set(filename)

    def start_conversion(self):
        zip_file = self.zip_path.get()
        iso_file = self.iso_path.get()

        if not zip_file or not os.path.exists(zip_file):
            messagebox.showerror("Error", "Please select a valid source Zip file.")
            return
        if not iso_file:
            messagebox.showerror("Error", "Please specify a destination ISO file.")
            return

        self.btn_convert.config(state='disabled')
        self.log("--- Starting Conversion ---")

        thread = threading.Thread(target=self.convert, args=(zip_file, iso_file), daemon=True)
        thread.start()

    def convert(self, zip_file, iso_file):
        temp_dir = tempfile.mkdtemp()
        try:
            # 1. Extract Zip
            self.log(f"Extracting {os.path.basename(zip_file)}...")
            with zipfile.ZipFile(zip_file, 'r') as zf:
                zf.extractall(temp_dir)
            self.log("Extraction complete.")

            # 2. Build ISO
            self.log("Initializing ISO image...")
            iso = pycdlib.PyCdlib()

            # Create ISO with Joliet (Windows) and Rock Ridge (Linux) support
            iso.new(interchange_level=3, joliet=3, rock_ridge='1.09', vol_ident='V_OS_BOOT')

            self.log("Adding files to ISO structure...")

            # Walk through the extracted directory
            # Base path in ISO is root '/'

            for root, dirs, files in os.walk(temp_dir):
                for dir_name in dirs:
                    full_path = os.path.join(root, dir_name)
                    rel_path = os.path.relpath(full_path, temp_dir)
                    iso_path = '/' + rel_path.replace(os.path.sep, '/')
                    try:
                        iso.add_directory(iso_path, joliet_path=iso_path, rr_name=dir_name)
                    except Exception as e:
                        self.log(f"[WARN] Skipped dir {rel_path}: {e}")

                for file_name in files:
                    full_path = os.path.join(root, file_name)
                    rel_path = os.path.relpath(full_path, temp_dir)
                    iso_path = '/' + rel_path.replace(os.path.sep, '/')
                    try:
                        iso.add_file(full_path, iso_path=iso_path, joliet_path=iso_path, rr_name=file_name)
                    except Exception as e:
                        self.log(f"[WARN] Failed to add {rel_path}: {e}")

            self.log("Writing ISO file (this may take a moment)...")
            iso.write(iso_file)
            iso.close()

            self.log(f"SUCCESS: ISO saved to {iso_file}")
            self.root.after(0, lambda: messagebox.showinfo("Done", "Conversion Complete!"))

        except Exception as e:
            self.log(f"CRITICAL ERROR: {str(e)}")
            self.root.after(0, lambda: messagebox.showerror("Error", str(e)))
        finally:
            self.log("Cleaning up temporary files...")
            shutil.rmtree(temp_dir, ignore_errors=True)
            self.root.after(0, lambda: self.btn_convert.config(state='normal'))

if __name__ == "__main__":
    root = tk.Tk()
    app = ZipToIsoConverter(root)
    root.mainloop()
