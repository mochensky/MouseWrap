# MouseWrap

**MouseWrap** is a lightweight, cross-platform utility that enables seamless mouse cursor wrapping around screen edges - similar to multi-monitor "toroidal" behavior, but on a single display.

When the cursor reaches any edge of the screen (within a configurable buffer zone), it instantly teleports to the opposite side, creating a continuous/wrapping desktop experience.

## Features

- Edge wrapping on all four sides (left ↔ right, top ↔ bottom)
- Configurable edge buffer (dead zone) in pixels
- Low CPU usage (~0–2%)
- No tray icon / no background service overhead
- Simple command-line interface
- Written in Rust using the `enigo` library

## Requirements

- Rust (1.65 or newer recommended)
- Windows, macOS or Linux with X11/Wayland support (enigo compatibility)

## Installation

```bash
# Clone the repository
git clone https://github.com/mochensky/MouseWrap.git
cd MouseWrap

# Build release version
cargo build --release

# The binary will be located at:
# ./target/release/mousewrap.exe (Windows)
```

## Usage

```bash
./mousewrap
```

On run the program will ask you:

```
Enter desired edge buffer size in pixels
↓ 
```

Just type a number (e.g. `5`) and press Enter.

- Buffer = 0 → wrapping activates exactly at the screen edge
- Buffer = 10 → cursor must reach 10 pixels from the edge to wrap

Press **Ctrl+C** in the terminal to exit.


## Building from source

```bash
# Debug build
cargo build

# Optimized release build (recommended)
cargo build --release
```
