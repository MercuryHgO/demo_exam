mod models;
mod views;

use eframe::egui::{self, Color32, Context, Rounding, Stroke};
use sqlx::{sqlite::SqliteConnectOptions, SqlitePool};
use views::{Views, ViewsData};

use std::str::FromStr;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let db = sqlx::SqlitePool::connect_lazy_with(
        SqliteConnectOptions::from_str("sqlite://data.sqlite").unwrap(),
    );

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc, db)))),
    )
    .unwrap();
}

struct History {
    previous: Vec<Views>,
    next: Vec<Views>,
}

// App
struct MyEguiApp {
    db: SqlitePool,
    history: History,
    current_view: Views,
    view_data: ViewsData
}

impl MyEguiApp {
    fn set_view(&mut self, view: Views) {
        self.history
            .previous
            .push(std::mem::replace(&mut self.current_view, view));
        self.history.next.clear();
    }

    fn go_back(&mut self) {
        match self.history.previous.last() {
            Some(_) => {
                let current_view = self.current_view;

                self.history.next.push(current_view.clone());


                self.current_view = if let Some(previous) = self.history.previous.pop() {
                    previous
                } else {
                    Views::MainView
                };
            }
            _ => (),
        }
    }

    fn go_forward(&mut self) {
        match self.history.next.last() {
            Some(_) => {
                let current_view = self.current_view;

                self.history.previous.push(current_view.clone());

                self.current_view = if let Some(next) = self.history.next.pop() {
                    next
                } else {
                    current_view.clone()
                };
            }
            _ => (),
        }
    }
}

impl MyEguiApp {
    fn new(cc: &eframe::CreationContext<'_>, db: sqlx::SqlitePool) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.

        cc.egui_ctx.set_visuals(egui::Visuals {
            dark_mode: false,
            widgets: egui::style::Widgets {
                inactive: egui::style::WidgetVisuals {
                    weak_bg_fill: Color32::from_hex("#67ba80").unwrap(),
                    bg_fill: Color32::from_gray(230),
                    bg_stroke: Default::default(), // separators, indentation lines
                    fg_stroke: Stroke::new(1.0, Color32::from_gray(60)), // normal text color
                    rounding: Rounding::same(2.0),
                    expansion: 0.0,
                },
                hovered: egui::style::WidgetVisuals {
                    weak_bg_fill: Color32::DARK_GREEN,
                    bg_fill: Color32::DARK_GREEN,
                    bg_stroke: Stroke::new(1.0, Color32::from_gray(105)), // e.g. hover over window edge or button
                    fg_stroke: Stroke::new(1.5, Color32::from_gray(240)),
                    rounding: Rounding::same(3.0),
                    expansion: 1.0,
                },
                ..egui::style::Widgets::light()
            },
            ..egui::Visuals::light()
        });

        Self {
            db,
            history: History {
                previous: vec![],
                next: vec![],
            },
            current_view: Views::MainView,
            view_data: ViewsData::default()
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        views::show(self, ctx);
    }
}

pub(crate) type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
enum Error {
    DatabaseError(sqlx::Error),
    Any(String)
}

impl Error {
    pub fn show(&self, ctx: &Context) {
        let mut open = true;
        egui::Window::new("Ошибка").open(&mut open).show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(self.to_string());
            });
        });
    }
}

impl From<sqlx::Error> for Error {
    fn from(value: sqlx::Error) -> Self {
        Self::DatabaseError(value)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::DatabaseError(err) => write!(f, "Ошибка при выполнении запроса:\n{}", err),
            Error::Any(msg) => write!(f, "{}",msg),
        }
    }
}

impl std::error::Error for Error {}
