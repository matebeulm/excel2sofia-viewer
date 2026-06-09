# excel2sofia-viewer

A desktop viewer for output files produced by [excel2sofia](https://github.com/matebeulm/excel2sofia). Loads one or more tab-separated `.dat` files and displays them as an interactive multi-series line plot.

## Features

- Open one or more `.dat` files via a file picker
- Multi-series plot with tab10 colour palette (cycles after 10 traces)
- Hover tooltip showing series name and X/Y coordinates
- Crosshairs and cursor position overlay
- Autoscale on load

## Installation

Download the latest release for your platform from the [Releases](https://github.com/matebeulm/excel2sofia-viewer/releases) page.

Alternatively, build from source (requires [Rust](https://rustup.rs)):

```
cargo build --release
```

The binary will be at `target/release/excel2sofia-viewer`.

## Usage

1. Launch the application.
2. Click **Open .dat files** and select one or more files.
3. The plot updates automatically. Hover over a line to see the series name and coordinates.

### Data file format

Files must be tab-separated with two columns: wavelength and value. One data point per line, no header.

```
400.0	0.1234
401.0	0.1256
...
```

## Configuration

On first run, a `config.toml` is created automatically at the platform config directory:

| Platform | Path |
|----------|------|
| Linux    | `~/.config/excel2sofia-viewer/config.toml` |
| macOS    | `~/Library/Application Support/excel2sofia-viewer/config.toml` |
| Windows  | `%APPDATA%\excel2sofia-viewer\config.toml` |

Example config:

```toml
# File extensions shown in the open dialog
file_extensions = ["dat"]

# Palette: list of [r, g, b] entries in 0.0–1.0 range. Repeats after the last entry.
palette = [
  [0.122, 0.467, 0.706],  # blue
  [1.000, 0.498, 0.055],  # orange
  ...
]
```
