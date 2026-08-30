<video src="https://private-user-images.githubusercontent.com/5970704/643234157-52cf9ea1-e842-4b8a-8dc7-1985d2c9c3b4.mp4?jwt=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpc3MiOiJnaXRodWIuY29tIiwiYXVkIjoicmF3LmdpdGh1YnVzZXJjb250ZW50LmNvbSIsImtleSI6ImtleTUiLCJleHAiOjE3ODgwOTc3NDIsIm5iZiI6MTc4ODA5NzQ0MiwicGF0aCI6Ii81OTcwNzA0LzY0MzIzNDE1Ny01MmNmOWVhMS1lODQyLTRiOGEtOGRjNy0xOTg1ZDJjOWMzYjQubXA0P1gtQW16LUFsZ29yaXRobT1BV1M0LUhNQUMtU0hBMjU2JlgtQW16LUNyZWRlbnRpYWw9QUtJQVZDT0RZTFNBNTNQUUs0WkElMkYyMDI2MDgzMCUyRnVzLWVhc3QtMSUyRnMzJTJGYXdzNF9yZXF1ZXN0JlgtQW16LURhdGU9MjAyNjA4MzBUMTM0NDAyWiZYLUFtei1FeHBpcmVzPTMwMCZYLUFtei1TaWduYXR1cmU9NWQ4YmYzNDI3YzVlNDE1NGRhMTAxZTBjZGJkNDMxMjgyYmQxNWQ2NDk0ZTVlZTEyOWQ4M2U0NWFmYmFmM2JhMiZYLUFtei1TaWduZWRIZWFkZXJzPWhvc3QmcmVzcG9uc2UtY29udGVudC10eXBlPXZpZGVvJTJGbXA0In0.VoCAhPNTPBpVXS_jC_lVMxFvotsvzB-dh5dkMGcnLsE" controls></video>

# DDC/CI OSD Manager
A rust desktop application to control monitor OSD settings (brightness, contrast, input source, color, and more) via DDC/CI, built with Rust, egui/eframe, and ddc-hi.

### Why
Most monitors expose their on-screen-display controls through DDC/CI over I2C, but adjusting them usually means grabbing the physical remote/buttons or shelling out to ddcutil by hand. This app gives you a native GUI to manage brightness, contrast, input source, and other VCP features across multiple monitors at once, without leaving your desktop.

### Features
- **Multi-monitor support**: detects all connected displays and lets you target one or several at once.
- **Capability-aware UI**: reads each monitor's declared VCP capabilities and only shows controls it actually supports, instead of a fixed hardcoded set.
- **Catppuccin theming**: Latte / Frappé / Macchiato / Mocha palettes, with a dark/light toggle built on top of egui::Visuals.
- **Background worker thread**: all DDC/CI I/O happens off the UI thread, since DDC/CI communication is inherently slow and blocking.

### How to Install
1. **Grant your user access to the I2C devices**
By default, /dev/i2c-* devices are only readable/writable by `root`. Rather than running the app as root, create an `i2c` group, add yourself to it, and set up a udev rule so the devices get the right group ownership automatically:
```bash
# Create the group (skip if it already exists)
sudo groupadd i2c

# Add your user to it
sudo usermod -aG i2c "$USER"

# Persist correct permissions via a udev rule
echo 'KERNEL=="i2c-[0-9]*", GROUP="i2c", MODE="0660"' | sudo tee /etc/udev/rules.d/45-i2c.rules

# Reload udev rules
sudo udevadm control --reload-rules
sudo udevadm trigger
```

**Log out and back in** for the new group membership to take effect; group changes don't apply to already-open sessions.

2. **Compile from source**
Before sources compilation, you need to install all dependencies. (WIP)
```bash
git clone <repo-url>
cd ddc-gui

# System libraries required to compile eframe/egui (X11/OpenGL bindings)
# Debian/Ubuntu:
sudo apt install pkg-config libx11-dev libxi-dev libxcursor-dev \
    libxrandr-dev libxkbcommon-dev libgl1-mesa-dev libwayland-dev

# Fedora:
sudo dnf install pkgconf-pkg-config libX11-devel libXi-devel \
    libXcursor-devel libXrandr-devel libxkbcommon-devel mesa-libGL-devel wayland-devel

# Arch:
sudo pacman -S pkgconf libx11 libxi libxcursor libxrandr libxkbcommon mesa wayland

cargo build --release
./target/release/lazy_osd
```

**Now move the bin under `cp ./target/release/lazy_osd /usr/local/sbin/lazy_osd`**.


2. Or **Install the app**
```bash
chmod +x DDC-CI-OSD-Manager-x86-64.AppImage
./DDC-CI-OSD-Manager-x86-64.AppImage
```

