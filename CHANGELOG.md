# Changelog

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
