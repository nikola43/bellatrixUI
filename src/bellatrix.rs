use crate::utils;
use eframe::egui::{
    self, Button, Color32, Context, FontData, FontDefinitions, FontFamily, Hyperlink, Label,
    Layout, Separator, TextStyle, TopBottomPanel, Ui, WidgetType,
};
// use std::borrow::Cow;
use std::iter::FromIterator;

pub const PADDING: f32 = 5.0;
const WHITE: Color32 = Color32::from_rgb(255, 255, 255);
const CYAN: Color32 = Color32::from_rgb(0, 255, 255);

pub struct BotLog {
    date: String,
    text: String,
    tx_hash: String,
}

#[derive(Default)]
pub struct Bellatrix {
    pub logs: Vec<BotLog>,

    pub private_key: String,

    pub address: String,

    // this how you opt-out of serialization of a member
    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_buy_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub force_sell_percent: f32,

    #[cfg_attr(feature = "persistence", serde(skip))]
    pub auto_swap: bool,
}

impl Bellatrix {
    const INTERNAL_SPACE: f32 = 5.0;

    pub fn new() -> Bellatrix {
        let iter = (0..20).map(|a| BotLog {
            date: format!("date{}", a),
            text: format!("{}", a),
            tx_hash: format!("{}", a),
        });
        Bellatrix {
            logs: Vec::from_iter(iter),
            address: String::new(),
            private_key: String::new(),
            force_buy_percent: 0.0,
            force_sell_percent: 0.0,
            auto_swap: false,
        }
    }

    /// render the wallet section
    pub fn render_wallet(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("Address:");
            // TODO(elsuizo:2022-02-25): validate the input
            let address_input = ui.text_edit_singleline(&mut self.address);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.horizontal(|ui| {
            ui.label("PrivateKey: ");
            // TODO(elsuizo:2022-02-25): validate the password
            let password_input = utils::password_ui(ui, &mut self.private_key);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_addres(&mut self, ui: &mut eframe::egui::Ui) {
        ui.horizontal(|ui| {
            ui.label("From(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            if ui.button("Accept").clicked() {
                println!("dsfsdf");
                // println!("{:?}", chrono::offset::Local::now());
            }
            // let password_input = utils::password_ui(ui, &mut self.private_key);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.horizontal(|ui| {
            ui.label("To(Address):");
            // TODO(elsuizo:2022-02-25): validate the address
            let address_input = ui.text_edit_singleline(&mut self.address);
            if ui.button("Scam").clicked() {
                println!("dsfsdf");
                // println!("{:?}", chrono::offset::Local::now());
            }
            // let password_input = utils::password_ui(ui, &mut self.private_key);
        });
        ui.add_space(Self::INTERNAL_SPACE);
        ui.separator();
    }

    pub fn render_new_log(&self, ui: &mut eframe::egui::Ui) {
        for a in &self.logs {
            ui.horizontal(|ui| {
                let title = format!("{}: {}", a.date, "Buy 12323 TKM - 0.23 BNB");
                ui.colored_label(WHITE, title);

                ui.style_mut().visuals.hyperlink_color = CYAN;
                ui.add_space(PADDING);
                ui.with_layout(Layout::right_to_left(), |ui| {
                    ui.add(Hyperlink::from_label_and_url(
                        &a.text,
                        "See Tx On Explorer ⤴",
                    ));
                });
            });
        }
    }
}
