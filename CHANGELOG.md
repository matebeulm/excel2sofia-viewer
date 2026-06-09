# Changelog

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
