use crate::models::partners;
use crate::models::product_types;
use crate::models::product_types::ProductType;
use crate::models::products;
use crate::models::products::Product;
use crate::models::sales;
use crate::models::sales::Sale;
use chrono::Datelike;
use eframe::egui::include_image;
use eframe::egui::ComboBox;
use eframe::egui::DragValue;
use eframe::egui::Image;
use eframe::egui::{self, Color32, RichText, Stroke};
use egui_extras::DatePickerButton;
use futures::executor::block_on;
use sqlx::types::time::Date;

use crate::{models::partners::Partner, MyEguiApp};

#[derive(Clone, Copy)]
pub enum Views {
    MainView,
    Partners,
    Sales,
    Products
}

#[derive(Clone)]
struct PartnersForm {
    partner_type: String,
    partner_name: String,
    director: String,
    email: String,
    phone: String,
    legal_address: String,
    inn: String,
    rating: i64,
}

struct PartnersViewData {
    error: (bool, String),
    form: (bool, PartnersForm),
}

#[derive(Clone)]
struct SalesForm {
    product: Option<Product>,
    quantity: i64,
    sale_date: chrono::NaiveDate,
    partner: Option<Partner>,
}

impl Default for SalesForm {
    fn default() -> Self {
        Self {
            product: None,
            quantity: 0,
            sale_date: chrono::NaiveDate::default(),
            partner: None,
        }
    }
}

struct SalesViewData {
    error: (bool, String),
    form: (bool, SalesForm),
}

pub struct ViewsData {
    partners_view: PartnersViewData,
    sales_views: SalesViewData,
    products_view: ProductsViewData
}

impl Default for ViewsData {
    fn default() -> Self {
        ViewsData {
            partners_view: PartnersViewData {
                error: (false, "".to_string()),
                form: (false, PartnersForm::default()),
            },
            sales_views: SalesViewData {
                error: (false, "".to_string()),
                form: (false, SalesForm::default()),
            },
            products_view: ProductsViewData::default()
        }
    }
}

impl Default for PartnersForm {
    fn default() -> Self {
        Self {
            partner_type: "".to_string(),
            partner_name: "".to_string(),
            director: "".to_string(),
            email: "".to_string(),
            phone: "".to_string(),
            legal_address: "".to_string(),
            inn: "".to_string(),
            rating: 0,
        }
    }
}

struct ProductsViewData {
    error: (bool, String),
    products_form: (bool, ProductsForm),
    product_types_form: (bool, ProductTypesForm)
}

impl Default for ProductsViewData {
    fn default() -> Self {
        Self {
            error: (false,"".to_string()),
            products_form: (false, ProductsForm {
                product_type: None,
                product_name: "".to_string(),
                article_number: "".to_string(),
                minimum_cost: 0
            }),
            product_types_form: (false, ProductTypesForm {
                product_type: "".to_string(),
                coefficient: 0.0
            })
        }
    }
}

struct ProductsForm {
    product_type: Option<ProductType>,
    product_name: String,
    article_number: String,
    minimum_cost: i64,
}

struct ProductTypesForm {
    product_type: String,
    coefficient: f64
}

pub fn show(app: &mut MyEguiApp, ctx: &egui::Context) {
    let res = || -> crate::Result<_> {
        egui_extras::install_image_loaders(ctx);

        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                if ui.button("<").clicked() {
                    app.go_back();
                };
                if ui.button(">").clicked() {
                    app.go_forward();
                };
            });
        });

        ctx.input(|i| {
            if i.pointer.button_clicked(egui::PointerButton::Extra1) {
                app.go_back();
            }
            if i.pointer.button_clicked(egui::PointerButton::Extra2) {
                app.go_forward();
            }
        });

        match app.current_view {
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
                            app.set_view(Views::Partners);
                        };
                        if ui.button(RichText::new("Продажи").size(20.0)).clicked() {
                            app.set_view(Views::Sales);
                        };
                        if ui.button(RichText::new("Товары").size(20.0)).clicked() {
                            app.set_view(Views::Products);
                        };
                    })
                });
            } // MainView
            Views::Partners => {
                let partners_list: Vec<Partner> = match block_on(partners::get_all(&app.db)) {
                    Ok(values) => values,
                    Err(e) => {
                        app.view_data.partners_view.error.0 = true;
                        app.view_data.partners_view.error.1 = e.to_string();
                        vec![]
                    }
                };

                ctx.send_viewport_cmd(egui::ViewportCommand::Title("Партнеры".into()));
                egui::SidePanel::right("partner_panel").show(ctx, |ui| {
                    if ui.button(RichText::new("Добавить").size(20.0)).clicked() {
                        app.view_data.partners_view.form.0 = true
                    };
                });

                let mut change_view = false;
                let form_opened = &mut app.view_data.partners_view.form.0;
                let form = &mut app.view_data.partners_view.form.1;
                egui::Window::new("Партнер")
                    .open(form_opened)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            // let mut form = PartnersForm::default();

                            ui.label("Название организации");
                            ui.text_edit_singleline(&mut form.partner_name);

                            ui.label("Тип организации");
                            ui.text_edit_singleline(&mut form.partner_type);

                            ui.label("Директор организации");
                            ui.text_edit_singleline(&mut form.director);

                            ui.label("E-Mail");
                            ui.text_edit_singleline(&mut form.email);

                            ui.label("Контактный номер");
                            ui.text_edit_singleline(&mut form.phone);

                            ui.label("Адресс");
                            ui.text_edit_singleline(&mut form.legal_address);

                            ui.label("ИНН");
                            ui.text_edit_singleline(&mut form.inn);

                            ui.label("Рейтинг");
                            ui.add(DragValue::new(&mut form.rating));

                            if ui.button(RichText::new("Внести").size(20.0)).clicked() {
                                let partner = Partner::new(
                                    form.partner_type.clone(),
                                    form.partner_name.clone(),
                                    form.director.clone(),
                                    form.email.clone(),
                                    form.phone.clone(),
                                    form.legal_address.clone(),
                                    form.inn.clone(),
                                    form.rating,
                                );
                                match block_on(partner.create(&app.db)) {
                                    Ok(_) => {
                                        change_view = true;
                                    }
                                    Err(e) => {
                                        app.view_data.partners_view.error.0 = true;
                                        app.view_data.partners_view.error.1 = e.to_string();
                                    }
                                }
                            };
                        })
                    });

                if change_view {
                    *form_opened = false;
                }

                let message = app.view_data.partners_view.error.1.clone();
                egui::Window::new("Ошибка")
                    .open(&mut app.view_data.partners_view.error.0)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(message);
                        })
                    });

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        partners_list.into_iter().for_each(|partner| {
                            egui::Frame::default()
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .inner_margin(0.4)
                                .outer_margin(0.8)
                                .show(ui, |ui| {
                                    ui.set_min_size((ui.available_width() - 0.2, 0.0).into());

                                    ui.collapsing(
                                        RichText::new(
                                            [&partner.partner_name, " | ", &partner.partner_type]
                                                .concat(),
                                        )
                                        .strong()
                                        .size(20.0),
                                        |ui| {
                                            if ui.button("Удалить").clicked() {
                                                match block_on(partner.delete(&app.db)) {
                                                    Ok(_res) => {}
                                                    Err(e) => {
                                                        app.view_data.partners_view.error.0 = true;
                                                        app.view_data.partners_view.error.1 =
                                                            e.to_string();
                                                    }
                                                }
                                            }
                                        },
                                    );

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Адресс: ").size(14.0));
                                        ui.label(partner.legal_address.clone());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Директор: ").size(14.0));
                                        ui.label(partner.director.clone());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("E-Mail: ").size(14.0));
                                        ui.label(partner.email.clone());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Телефон: ").size(14.0));
                                        ui.label(partner.phone.clone());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("ИНН: ").size(14.0));
                                        ui.label(partner.inn.clone());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Рейтинг: ").size(14.0));
                                        ui.label(partner.rating.to_string());
                                    });
                                });
                        });
                    });
                });
            } // PartnersView
            Views::Sales => {
                let sales = match block_on(sales::get_all(&app.db)) {
                    Ok(sales) => sales,
                    Err(e) => {
                        app.view_data.sales_views.error.0 = true;
                        app.view_data.sales_views.error.1 = e.to_string();
                        vec![]
                    }
                };

                let partners_list = match block_on(partners::get_all(&app.db)) {
                    Ok(values) => values,
                    Err(e) => {
                        app.view_data.sales_views.error.0 = true;
                        app.view_data.sales_views.error.1 = e.to_string();
                        vec![]
                    }
                };

                let products_list = match block_on(products::get_all(&app.db)) {
                    Ok(values) => values,
                    Err(e) => {
                        app.view_data.sales_views.error.0 = true;
                        app.view_data.sales_views.error.1 = e.to_string();
                        vec![]
                    }
                };

                ctx.send_viewport_cmd(egui::ViewportCommand::Title("Продажи".into()));
                egui::SidePanel::right("sales_panel").show(ctx, |ui| {
                    if ui.button(RichText::new("Добавить").size(20.0)).clicked() {
                        app.view_data.sales_views.form.0 = true
                    };
                });

                let mut change_view = false;
                let form_opened = &mut app.view_data.sales_views.form.0;
                let form = &mut app.view_data.sales_views.form.1;
                egui::Window::new("Продажа")
                    .open(form_opened)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Продукт");
                            ComboBox::from_id_salt("products_combo_box")
                                .selected_text(
                                    &form.product.clone().unwrap_or_default().product_name,
                                )
                                .show_ui(ui, |ui| {
                                    products_list.into_iter().for_each(|product| {
                                        let name =
                                            [&product.product_name, " | ", &product.product_type]
                                                .concat();
                                        ui.selectable_value(&mut form.product, Some(product), name);
                                    })
                                });

                            ui.label("Количество");
                            ui.add(DragValue::new(&mut form.quantity));

                            ui.label("Дата прдажи");
                            ui.add(DatePickerButton::new(&mut form.sale_date));

                            ui.label("Партнер");
                            ComboBox::from_id_salt("partners_combo_box")
                                .selected_text(
                                    &form.partner.clone().unwrap_or_default().partner_name,
                                )
                                .show_ui(ui, |ui| {
                                    partners_list.into_iter().for_each(|partner| {
                                        let name =
                                            [&partner.partner_name, " | ", &partner.partner_type]
                                                .concat();
                                        ui.selectable_value(&mut form.partner, Some(partner), name);
                                    })
                                });

                            if ui.button(RichText::new("Внести").size(20.0)).clicked() {
                                let transaction = || -> crate::Result<()> {
                                    let partner = Sale::new(
                                        form.product.clone().unwrap_or_default().id,
                                        form.quantity.clone(),
                                        Date::from_ordinal_date(
                                            form.sale_date.year_ce().1 as i32,
                                            form.sale_date.ordinal0().try_into().unwrap(),
                                        )
                                        .map_err(|e| crate::Error::Any(e.to_string()))?,
                                        form.partner.clone().unwrap_or_default().id,
                                    );

                                    block_on(partner.create(&app.db))?;

                                    Ok(())
                                };

                                match transaction() {
                                    Ok(_) => {
                                        change_view = true;
                                    }
                                    Err(e) => {
                                        app.view_data.sales_views.error.0 = true;
                                        app.view_data.sales_views.error.1 = e.to_string();
                                    }
                                }
                            };
                        })
                    });

                if change_view {
                    *form_opened = false;
                }

                let message = app.view_data.partners_view.error.1.clone();
                egui::Window::new("Ошибка")
                    .open(&mut app.view_data.partners_view.error.0)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(message);
                        })
                    });

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        sales.into_iter().for_each(|sale| {
                            egui::Frame::default()
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .inner_margin(0.4)
                                .outer_margin(0.8)
                                .show(ui, |ui| {
                                    ui.set_min_size((ui.available_width() - 0.2, 0.0).into());

                                    let name = match block_on(products::get(&app.db, sale.product_id.clone())) {
                                        Ok(product) => product.product_name,
                                        Err(_) => "Ошибка".to_string(),
                                    };
                                    
                                    ui.collapsing(
                                        RichText::new(name).strong().size(20.0),
                                        |ui| {
                                            if ui.button("Удалить").clicked() {
                                                match block_on(sale.delete(&app.db)) {
                                                    Ok(_res) => {}
                                                    Err(e) => {
                                                        app.view_data.partners_view.error.0 = true;
                                                        app.view_data.partners_view.error.1 =
                                                            e.to_string();
                                                    }
                                                }
                                            }
                                        },
                                    );

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Количество: ").size(14.0));
                                        ui.label(sale.quantity.to_string());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Дата: ").size(14.0));
                                        ui.label(sale.sale_date.to_string());
                                    });

                                    ui.horizontal(|ui| {
                                        let partner: String =
                                            match block_on(partners::get(&app.db, sale.partner_id))
                                            {
                                                Ok(partner) => [
                                                    &partner.partner_name,
                                                    " | ",
                                                    &partner.partner_type,
                                                ]
                                                .concat()
                                                .to_string(),
                                                Err(_) => "Ошибка".into(),
                                            };
                                        ui.label(RichText::new("Партнер: ").size(14.0));
                                        ui.label(partner);
                                    });
                                });
                        });
                    });
                });
            }, // SalesView
            Views::Products => {
                let products_list = match block_on(products::get_all(&app.db)) {
                    Ok(values) => values,
                    Err(e) => {
                        app.view_data.products_view.error.0 = true;
                        app.view_data.products_view.error.1 = e.to_string();
                        vec![]
                    }
                };

                let product_types = match block_on(product_types::get_all(&app.db)) {
                    Ok(values) => values,
                    Err(e) => {
                        app.view_data.products_view.error.0 = true;
                        app.view_data.products_view.error.1 = e.to_string();
                        vec![]
                    }
                };

                ctx.send_viewport_cmd(egui::ViewportCommand::Title("Продукты".into()));
                egui::SidePanel::right("sales_panel").show(ctx, |ui| {
                    if ui.button(RichText::new("Добавить").size(20.0)).clicked() {
                        app.view_data.products_view.products_form.0 = true
                    };
                    if ui.button(RichText::new("Новый тип").size(20.0)).clicked() {
                        app.view_data.products_view.product_types_form.0 = true
                    };
                });

                let mut change_view = false;

                let product_form_opened = &mut app.view_data.products_view.products_form.0;
                let product_form = &mut app.view_data.products_view.products_form.1;


                let product_type_opened = &mut app.view_data.products_view.product_types_form.0;
                let product_type = &mut app.view_data.products_view.product_types_form.1;
                egui::Window::new("Продукт")
                    .open(product_form_opened)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Тип");
                            ComboBox::from_id_salt("products_combo_box")
                                .selected_text(
                                    &product_form.product_type.clone().unwrap_or_default().product_type,
                                )
                                .show_ui(ui, |ui| {
                                    product_types.clone().into_iter().for_each(|product| {
                                        let name = product.product_type.clone();
                                        ui.selectable_value(&mut product_form.product_type, Some(product), name);
                                    })
                                });

                            ui.label("Название");
                            ui.text_edit_singleline(&mut product_form.product_name);

                            ui.label("Артикул");
                            ui.text_edit_singleline(&mut product_form.article_number);

                            ui.label("Минимальная цена");
                            ui.add(DragValue::new(&mut product_form.minimum_cost));

                            if ui.button(RichText::new("Внести").size(20.0)).clicked() {
                                let transaction = || -> crate::Result<()> {
                                    let product = Product::new(
                                        product_form.product_type.clone().unwrap_or_default().product_type,
                                        product_form.product_name.clone(),
                                        product_form.article_number.clone(),
                                        product_form.minimum_cost.clone()
                                    );

                                    block_on(product.create(&app.db))?;

                                    Ok(())
                                };

                                match transaction() {
                                    Ok(_) => {
                                        change_view = true;
                                    }
                                    Err(e) => {
                                        app.view_data.products_view.error.0 = true;
                                        app.view_data.products_view.error.1 = e.to_string();
                                    }
                                }
                            };
                        })
                    });


                egui::Window::new("Тип продукта")
                    .open(product_type_opened)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label("Тип");

                            ui.label("Название");
                            ui.text_edit_singleline(&mut product_type.product_type);

                            ui.label("Коэффицент");
                            ui.add(DragValue::new(&mut product_type.coefficient));

                            if ui.button(RichText::new("Внести").size(20.0)).clicked() {
                                let transaction = || -> crate::Result<()> {
                                    let product_type = ProductType::new(
                                        product_type.product_type.clone(),
                                        product_type.coefficient.clone()
                                    );

                                    block_on(product_type.create(&app.db))?;

                                    Ok(())
                                };

                                match transaction() {
                                    Ok(_) => {
                                        change_view = true;
                                    }
                                    Err(e) => {
                                        app.view_data.products_view.error.0 = true;
                                        app.view_data.products_view.error.1 = e.to_string();
                                    }
                                }
                            };
                        })
                    });


                if change_view {
                    *product_form_opened = false;
                }

                let message = app.view_data.products_view.error.1.clone();
                egui::Window::new("Ошибка")
                    .open(&mut app.view_data.products_view.error.0)
                    .show(ctx, |ui| {
                        ui.vertical_centered(|ui| {
                            ui.label(message);
                        })
                    });

                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical()
                        .id_salt("products_scroll")
                        .max_height(ui.available_height() / 2.0)
                        .show(ui, |ui| {
                        products_list.into_iter().for_each(|product| {
                            egui::Frame::default()
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .inner_margin(0.4)
                                .outer_margin(0.8)
                                .show(ui, |ui| {
                                    ui.set_min_size((ui.available_width() - 0.2, 0.0).into());

                                    ui.collapsing(
                                        RichText::new(product.product_name.clone()).strong().size(20.0),
                                        |ui| {
                                            if ui.button("Удалить").clicked() {
                                                match block_on(product.delete(&app.db)) {
                                                    Ok(_res) => {}
                                                    Err(e) => {
                                                        app.view_data.partners_view.error.0 = true;
                                                        app.view_data.partners_view.error.1 =
                                                            e.to_string();
                                                    }
                                                }
                                            }
                                        },
                                    );

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Тип: ").size(14.0));
                                        ui.label(product.product_type.to_string());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Артикул: ").size(14.0));
                                        ui.label(product.article_number.to_string());
                                    });

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Минимальная цена: ").size(14.0));
                                        ui.label(product.minimum_cost.to_string());
                                    });
                                });
                        });
                    });

                    ui.separator();

                    egui::ScrollArea::vertical().id_salt("types_scroll").show(ui, |ui| {
                        product_types.into_iter().for_each(|product_type| {
                            egui::Frame::default()
                                .stroke(Stroke::new(1.0, Color32::GRAY))
                                .inner_margin(0.4)
                                .outer_margin(0.8)
                                .show(ui, |ui| {
                                    ui.set_min_size((ui.available_width() - 0.2, 0.0).into());

                                    ui.collapsing(
                                        RichText::new(product_type.product_type.clone()).strong().size(20.0),
                                        |ui| {
                                            if ui.button("Удалить").clicked() {
                                                match block_on(product_type.delete(&app.db)) {
                                                    Ok(_res) => {}
                                                    Err(e) => {
                                                        app.view_data.products_view.error.0 = true;
                                                        app.view_data.products_view.error.1 = e.to_string();
                                                    }
                                                }
                                            }
                                        },
                                    );

                                    ui.horizontal(|ui| {
                                        ui.label(RichText::new("Коэффицент: ").size(14.0));
                                        ui.label(product_type.coefficient.to_string());
                                    });
                                });
                        });
                    });
                });
                
            } // ProductsView
        };

        Ok(())
    }();

    match res {
        Err(error) => {
            let mut open = true;
            egui::Window::new("Ошибка").open(&mut open).show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.label(error.to_string());
                });
            });
        }
        _ => (),
    };
}
