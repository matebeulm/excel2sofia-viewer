# Changelog

## [0.3.6] - 2026-06-10

### Added
- 3-column `.dat` files: third column is plotted as a dashed fit curve against the same x-axis, with `_fit` appended to the legend name

### Fixed
- Comment lines starting with `#` are now skipped when loading `.dat` files
- Column separator now accepts any whitespace (tabs and spaces), not just tabs

## [0.3.5] - 2026-06-09

### Added
- Configurable x/y axis labels (default: "wavelength [nm]" and "transmission"); set via `x_label` and `y_label` in `config.toml`

## [0.3.4] - 2026-06-09

### Added
- Catppuccin theme selector (Latte, Frappé, Macchiato, Mocha) in the toolbar; selection persists to `config.toml`

## [0.3.3] - 2026-06-09

### Changed
- App now follows the OS dark/light theme preference and updates live when the system theme changes

## [0.3.2] - 2026-06-09

### Added
- "Open folder" button loads all matching files from a selected directory, sorted alphabetically — works around the macOS Cmd+A file dialog limitation

## [0.3.1] - 2026-06-09

### Added
- macOS `.dmg` installer containing a proper `.app` bundle — install by dragging to `/Applications`, making the app visible to Spotlight
- App icon (spectral peak design) shown in the dock, taskbar, and Windows Explorer
- Ad-hoc code signature on the macOS `.app` bundle

## [0.3.0] - 2026-06-09

### Changed
- Replaced iced + iced_plot with egui (eframe + egui_plot), enabling more capable plotting
- `line_width` from `config.toml` is now applied to rendered lines (was previously unused)
- Removed proportional downsampling workaround — no longer needed with egui_plot

### Added
- MSI installer for Windows releases
- macOS `.pkg` installer for macOS releases

### Removed
- Windows ARM64 (`aarch64-pc-windows-msvc`) release target — x86_64 MSI runs via emulation on ARM64 Windows

## [0.2.0] - 2026-06-09

### Added
- README with installation, usage, and configuration documentation

### Fixed
- Console window no longer appears on Windows

## [0.1.1] - 2026-06-09

### Added
- Windows ARM64 (`aarch64-pc-windows-msvc`) release target
- Config file is now created automatically on first run at the platform config directory (`~/.config/excel2sofia-viewer/config.toml` on Linux, `~/Library/Application Support/excel2sofia-viewer/config.toml` on macOS, `%APPDATA%\excel2sofia-viewer\config.toml` on Windows)

## [0.1.0] - 2026-06-09

### Added
- Initial release
- Load one or more `.dat` files (tab-separated wavelength/value) via file picker
- Multi-series plot with tab10 colour palette, cycling after 10 traces
- Autoscale on load
- Hover tooltip showing series name and X/Y coordinates
- Crosshairs and cursor position overlay
- `config.toml` for palette and file extension configuration
- Proportional downsampling to keep hover working with many series (workaround for GPU picking on macOS)
