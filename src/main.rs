#![windows_subsystem = "windows"]

use std::path::Path;

use eframe::egui;
use egui_plot::{Legend, Line, Plot, PlotPoints};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Copy, Default)]
#[serde(rename_all = "lowercase")]
enum ThemeSetting {
    #[default]
    System,
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

#[derive(Serialize, Deserialize)]
struct Config {
    line_width: f32,
    file_extensions: Vec<String>,
    palette: Vec<[f32; 3]>,
    #[serde(default)]
    theme: ThemeSetting,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            line_width: 3.0,
            file_extensions: vec!["dat".to_string()],
            theme: ThemeSetting::default(),
            palette: vec![
                [0.122, 0.467, 0.706],
                [1.000, 0.498, 0.055],
                [0.173, 0.627, 0.173],
                [0.839, 0.153, 0.157],
                [0.580, 0.404, 0.741],
                [0.549, 0.337, 0.294],
                [0.890, 0.467, 0.761],
                [0.498, 0.498, 0.498],
                [0.737, 0.741, 0.133],
                [0.090, 0.745, 0.812],
            ],
        }
    }
}

fn config_path() -> Option<std::path::PathBuf> {
    dirs::config_dir().map(|d| d.join("excel2sofia-viewer").join("config.toml"))
}

fn save_config(config: &Config) {
    let Some(path) = config_path() else { return };
    let content = toml::to_string(config).expect("config serialization failed");
    let _ = std::fs::write(path, content);
}

fn load_config() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };
    if path.exists() {
        std::fs::read_to_string(&path)
            .ok()
            .and_then(|s| toml::from_str(&s).ok())
            .unwrap_or_default()
    } else {
        let config = Config::default();
        if let Some(dir) = path.parent() {
            let _ = std::fs::create_dir_all(dir);
        }
        let content = toml::to_string(&config).expect("config serialization failed");
        let _ = std::fs::write(&path, content);
        config
    }
}

struct Series {
    name: String,
    points: Vec<[f64; 2]>,
    color: egui::Color32,
}

struct App {
    series: Vec<Series>,
    config: Config,
    reset_bounds: bool,
}

impl App {
    fn new() -> Self {
        App {
            series: Vec::new(),
            config: load_config(),
            reset_bounds: false,
        }
    }

    fn open_files(&mut self) {
        let exts: Vec<&str> = self.config.file_extensions.iter().map(String::as_str).collect();
        let Some(paths) = rfd::FileDialog::new()
            .add_filter("data files", &exts)
            .pick_files()
        else {
            return;
        };
        self.load_paths(paths);
    }

    fn open_folder(&mut self) {
        let Some(folder) = rfd::FileDialog::new().pick_folder() else {
            return;
        };
        let exts = &self.config.file_extensions;
        let paths: Vec<_> = std::fs::read_dir(&folder)
            .into_iter()
            .flatten()
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension()
                    .and_then(|e| e.to_str())
                    .map(|e| exts.iter().any(|x| x.eq_ignore_ascii_case(e)))
                    .unwrap_or(false)
            })
            .collect();
        self.load_paths(paths);
    }

    fn load_paths(&mut self, mut paths: Vec<std::path::PathBuf>) {
        paths.sort();
        let palette = &self.config.palette;
        let loaded: Vec<Series> = paths
            .iter()
            .enumerate()
            .filter_map(|(i, p)| {
                let name = p.file_stem()?.to_string_lossy().into_owned();
                let points = load_dat(p).ok()?;
                let [r, g, b] = palette[i % palette.len()];
                let color = egui::Color32::from_rgb(
                    (r * 255.0) as u8,
                    (g * 255.0) as u8,
                    (b * 255.0) as u8,
                );
                Some(Series { name, points, color })
            })
            .collect();

        if !loaded.is_empty() {
            self.series = loaded;
            self.reset_bounds = true;
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        match self.config.theme {
            ThemeSetting::System => ctx.set_theme(egui::ThemePreference::System),
            ThemeSetting::Latte => catppuccin_egui::set_theme(ctx, catppuccin_egui::LATTE),
            ThemeSetting::Frappe => catppuccin_egui::set_theme(ctx, catppuccin_egui::FRAPPE),
            ThemeSetting::Macchiato => catppuccin_egui::set_theme(ctx, catppuccin_egui::MACCHIATO),
            ThemeSetting::Mocha => catppuccin_egui::set_theme(ctx, catppuccin_egui::MOCHA),
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                if ui.button("Open .dat files").clicked() {
                    self.open_files();
                }
                if ui.button("Open folder").clicked() {
                    self.open_folder();
                }
                let theme_before = self.config.theme;
                egui::ComboBox::from_id_salt("theme")
                    .selected_text(match self.config.theme {
                        ThemeSetting::System => "System",
                        ThemeSetting::Latte => "Latte",
                        ThemeSetting::Frappe => "Frappé",
                        ThemeSetting::Macchiato => "Macchiato",
                        ThemeSetting::Mocha => "Mocha",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.config.theme, ThemeSetting::System, "System");
                        ui.selectable_value(&mut self.config.theme, ThemeSetting::Latte, "Latte");
                        ui.selectable_value(&mut self.config.theme, ThemeSetting::Frappe, "Frappé");
                        ui.selectable_value(&mut self.config.theme, ThemeSetting::Macchiato, "Macchiato");
                        ui.selectable_value(&mut self.config.theme, ThemeSetting::Mocha, "Mocha");
                    });
                if self.config.theme != theme_before {
                    save_config(&self.config);
                }
            });
            ui.add_space(10.0);

            let mut plot = Plot::new("spectrum")
                .legend(Legend::default())
                .label_formatter(|name, value| {
                    if name.is_empty() {
                        format!("X: {:.1}  Y: {:.4}", value.x, value.y)
                    } else {
                        format!("{}\nX: {:.1}  Y: {:.4}", name, value.x, value.y)
                    }
                })
                .height(ui.available_height());

            if self.reset_bounds {
                self.reset_bounds = false;
                plot = plot.reset();
            }

            let line_width = self.config.line_width;
            plot.show(ui, |plot_ui| {
                for series in &self.series {
                    plot_ui.line(
                        Line::new(PlotPoints::new(series.points.clone()))
                            .name(&series.name)
                            .color(series.color)
                            .width(line_width),
                    );
                }
            });
        });
    }
}

fn load_icon() -> egui::IconData {
    let bytes = include_bytes!("../icons/icon_256.png");
    let img = image::load_from_memory(bytes).expect("invalid icon").into_rgba8();
    let (w, h) = img.dimensions();
    egui::IconData { rgba: img.into_raw(), width: w, height: h }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("excel2sofia viewer")
            .with_icon(load_icon()),
        ..Default::default()
    };
    eframe::run_native(
        "excel2sofia viewer",
        options,
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "FiraCode".to_owned(),
                std::sync::Arc::new(egui::FontData::from_static(
                    include_bytes!("../fonts/FiraCodeNerdFont-Regular.ttf"),
                )),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "FiraCode".to_owned());
            fonts
                .families
                .entry(egui::FontFamily::Monospace)
                .or_default()
                .insert(0, "FiraCode".to_owned());
            cc.egui_ctx.set_fonts(fonts);
            Ok(Box::new(App::new()))
        }),
    )
    .expect("failed to run app");
}

fn load_dat(path: &Path) -> Result<Vec<[f64; 2]>, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path)?;
    let data = content
        .lines()
        .filter_map(|line| {
            let mut parts = line.split('\t');
            let x: f64 = parts.next()?.trim().parse().ok()?;
            let y: f64 = parts.next()?.trim().parse().ok()?;
            Some([x, y])
        })
        .collect();
    Ok(data)
}
