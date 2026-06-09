use std::path::Path;

use iced::{
    Element,
    Length::Fill,
    Task,
    widget::{button, column},
};
use iced_plot::{LineStyle, MarkerStyle, PlotUiMessage, PlotWidget, PlotWidgetBuilder, Series, ShapeId};
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    /// Reserved for future use — iced_plot does not yet expose line width.
    #[allow(dead_code)]
    line_width: f32,
    file_extensions: Vec<String>,
    palette: Vec<[f32; 3]>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            line_width: 3.0,
            file_extensions: vec!["dat".to_string()],
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

fn load_config() -> Config {
    std::fs::read_to_string("config.toml")
        .ok()
        .and_then(|s| toml::from_str(&s).ok())
        .unwrap_or_default()
}

fn main() -> iced::Result {
    iced::application(App::new, App::update, App::view)
        .title("excel2sofia viewer")
        .font(include_bytes!("../fonts/FiraCodeNerdFont-Regular.ttf"))
        .default_font(iced::Font::with_name("FiraCode Nerd Font"))
        .run()
}

struct App {
    plot: PlotWidget,
    series_ids: Vec<ShapeId>,
    config: Config,
}

#[derive(Debug, Clone)]
enum Message {
    Plot(PlotUiMessage),
    OpenFile,
    FilesLoaded(Vec<(String, Vec<[f64; 2]>)>),
}

impl App {
    fn new() -> (Self, Task<Message>) {
        let config = load_config();
        let plot = PlotWidgetBuilder::new()
            .with_x_label("wavelength")
            .with_y_label("value")
            .with_crosshairs(true)
            .with_cursor_overlay(true)
            .with_cursor_provider(|x, y| format!("X: {x:.1}  Y: {y:.4}"))
            .with_hover_radius_px(20.0)
            .with_hover_highlight_provider(|ctx, point| {
                point.marker_style = Some(MarkerStyle::circle(6.0));
                Some(format!("{}\nX: {:.1}  Y: {:.4}", ctx.series_label, point.x, point.y))
            })
            .with_autoscale_on_updates(true)
            .build()
            .unwrap();
        (App { plot, series_ids: Vec::new(), config }, Task::none())
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Plot(m) => {
                self.plot.update(m);
                Task::none()
            }
            Message::OpenFile => {
                let exts: Vec<&str> = self.config.file_extensions.iter().map(String::as_str).collect();
                if let Some(paths) = rfd::FileDialog::new()
                    .add_filter("data files", &exts)
                    .pick_files()
                {
                    let mut loaded: Vec<(String, Vec<[f64; 2]>)> = paths
                        .iter()
                        .filter_map(|p| {
                            let name = p.file_stem()?.to_string_lossy().into_owned();
                            let data = load_dat(p).ok()?;
                            Some((name, data))
                        })
                        .collect();
                    // iced_plot switches to GPU picking above 5000 total points (broken on
                    // macOS Metal). Downsample proportionally to stay under the threshold.
                    const CPU_PICK_THRESHOLD: usize = 4800;
                    let total: usize = loaded.iter().map(|(_, d)| d.len()).sum();
                    if total > CPU_PICK_THRESHOLD {
                        let keep = (CPU_PICK_THRESHOLD / loaded.len()).max(1);
                        for (_, data) in &mut loaded {
                            if data.len() > keep {
                                let step = (data.len() as f64 / keep as f64).ceil() as usize;
                                *data = data.iter().copied().step_by(step).collect();
                            }
                        }
                    }
                    if !loaded.is_empty() {
                        return Task::done(Message::FilesLoaded(loaded));
                    }
                }
                Task::none()
            }
            Message::FilesLoaded(files) => {
                for id in &self.series_ids {
                    let _ = self.plot.remove_series(id);
                }
                self.series_ids.clear();
                let palette = &self.config.palette;
                for (i, (name, data)) in files.into_iter().enumerate() {
                    let color = palette[i % palette.len()];
                    let series = Series::line_only(data, LineStyle::Solid)
                        .with_label(&name)
                        .with_color(color);
                    self.series_ids.push(series.id);
                    let _ = self.plot.add_series(series);
                }
                Task::none()
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            button("Open .dat files").on_press(Message::OpenFile),
            self.plot.view().map(Message::Plot),
        ]
        .spacing(10)
        .padding(10)
        .width(Fill)
        .height(Fill)
        .into()
    }
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
