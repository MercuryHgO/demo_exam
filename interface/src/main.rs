mod partners;

use core::error;
use std::{borrow::Borrow, sync::{Arc, Mutex}};

use eframe::egui::{self, include_image, Color32, Context, Image, RichText, Rounding, ScrollArea, Stroke};
use partners::Partner;
use sqlx::{sqlite::{SqliteConnectOptions, SqlitePoolOptions}, ConnectOptions, SqlitePool};

use std::str::FromStr;

fn main() {
    let native_options = eframe::NativeOptions::default();

    let db = sqlx::SqlitePool::connect_lazy_with(
        SqliteConnectOptions::from_str("sqlite://data.sqlite").unwrap()
    );

    eframe::run_native(
        "My egui App",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc, db)))),
    )
    .unwrap();
}

#[derive(Clone)]
struct PartnersForm {
    is_opened: bool,
    name: String,
    partnet_type: String,
    director: String,
}

impl Default for PartnersForm {
    fn default() -> Self {
        Self {
            is_opened: false,
            name: "".to_string(),
            partnet_type: "".to_string(),
            director: "".to_string(),
        }
    }
}

#[derive(Clone)]
enum Views {
    MainView,
    Partners {
        error: (bool,String),
        partners_list: Vec<Partner>,
        form: PartnersForm,
    },
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
                self.history.next.push(self.current_view.clone());

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
                self.history.previous.push(self.current_view.clone());

                self.current_view = if let Some(next) = self.history.next.pop() {
                    next
                } else {
                    self.current_view.clone()
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
            ..Default::default()
        });

        Self {
            db,
            history: History {
                previous: vec![],
                next: vec![],
            },
            current_view: Views::MainView,
        }
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let res = || -> Result<_> {
            egui_extras::install_image_loaders(ctx);

            egui::TopBottomPanel::top("header").show(ctx, |ui| {
                ui.horizontal_centered(|ui| {
                    if ui.button("<").clicked() {
                        self.go_back();
                    };
                    if ui.button(">").clicked() {
                        self.go_forward();
                    };
                });
            });

            ctx.input(|i| {
                if i.pointer.button_clicked(egui::PointerButton::Extra1) {
                    self.go_back();
                }
                if i.pointer.button_clicked(egui::PointerButton::Extra2) {
                    self.go_forward();
                }
            });

            match &mut self.current_view {
                Views::MainView => {
                    ctx.send_viewport_cmd(egui::ViewportCommand::Title("Главное меню".into()));
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.add(
                                Image::new(include_image!("resources/logo.png"))
                                    .max_width(200.0)
                                    .max_height(200.0),
                            );
                            if ui.button(RichText::new("Партнеры").size(20.0)).clicked() {
                                let mut error = (false, "".to_string());

                                let partners_list = match futures::executor::block_on( partners::get_all(&self.db) ) {
                                    Ok(fetched) => fetched,
                                    Err(e) => {
                                        error.0 = true;
                                        error.1 = e.to_string();
                                        vec![]
                                    },
                                };



                                self.set_view(Views::Partners {
                                    partners_list,
                                    form: PartnersForm::default(),
                                    error
                                })
                            };
                        })
                    });
                }
                Views::Partners {
                    form,
                    partners_list,
                    error
                } => {

                    ctx.send_viewport_cmd(egui::ViewportCommand::Title("Партнеры".into()));
                    egui::SidePanel::right("partner_panel").show(ctx, |ui| {
                        if ui.button(RichText::new("Добавить").size(20.0)).clicked() {
                            form.is_opened = true
                        };
                    });

                    egui::Window::new("Партнер")
                        .open(&mut form.is_opened)
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label("Название организации");
                                ui.text_edit_singleline(&mut form.name);

                                ui.label("Тип организации");
                                ui.text_edit_singleline(&mut form.partnet_type);

                                ui.label("Директор организации");
                                ui.text_edit_singleline(&mut form.director);

                                ui.button(RichText::new("Внести").size(20.0));
                            })
                        });

                    let message = error.1.clone();
                    egui::Window::new("Ошибка")
                        .open(&mut error.0)
                        .show(ctx, |ui| {
                            ui.vertical_centered(|ui| {
                                ui.label(message);
                            })
                        });

                    egui::CentralPanel::default().show(ctx, |ui| {
                        egui::ScrollArea::vertical()
                            .show(ui, |ui| {
                                partners_list
                                    .into_iter()
                                    .for_each(|partner| {
                                        egui::Frame::default()
                                            .stroke(Stroke::new(1.0, Color32::GRAY))
                                            .inner_margin(0.4)
                                            .outer_margin(0.8)
                                            .show(ui, |ui| {
                                                ui.set_min_size((ui.available_width() - 0.2, 0.0).into());

                                    
                                                ui.horizontal(|ui| {
                                                    ui.label( // Distribute them to the sides
                                                        RichText::new(partner.partner_name.clone())
                                                            .strong()
                                                            .size(20.0)
                                                    );

                                                    ui.separator();
                                        
                                                    ui.label( // Distibte them to the sides
                                                        RichText::new(partner.partner_type.clone())
                                                            .strong()
                                                            .size(20.0)
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("Адресс: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.legal_address.clone()
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("Директор: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.director.clone()
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("E-Mail: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.email.clone()
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("Телефон: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.phone.clone()
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("ИНН: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.inn.clone()
                                                    );
                                                });

                                                ui.horizontal(|ui| {
                                                    ui.label(
                                                        RichText::new("Рейтинг: ")
                                                            .size(14.0)
                                                    );
                                                    ui.label(
                                                        partner.rating.to_string()
                                                    );
                                                });
                                            });
                                    });
                            });
                        });
                }
            };
            Ok(())
        }();

        match res {
            Err(error) => {
                let mut open = true;
                egui::Window::new("Ошибка")
                    .open(&mut open)
                    .show(ctx, |ui| {
                        ui.vertical(|ui| {
                            ui.label(error.to_string());
                        });
                });
            },
            _ => ()
        };
    }
}

type Result<T> = std::result::Result<T, Error>;


#[derive(Debug)]
enum Error {
    DatabaseError(sqlx::Error),
}

impl Error {
    pub fn show(&self, ctx: &Context) {
        let mut open = true;
        egui::Window::new("Ошибка")
            .open(&mut open)
            .show(ctx, |ui| {
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
            Error::DatabaseError(err) => write!(f,"Ошибка при выполнении запроса:\n{}",err),
        }
    }
}

impl std::error::Error for Error { }
