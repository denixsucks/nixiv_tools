use crate::errlog;

#[cfg(not(target_arch = "wasm32"))]
use crate::Runtime;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NixivApp {
    label: String,
    information: String,
}

impl Default for NixivApp {
    fn default() -> Self {
        Self {
            label: "".to_owned(),
            information: "".to_owned(),
        }
    }
}

impl NixivApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for NixivApp {
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            draw_header(self, ctx, ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            draw_content(self, ctx, ui);
            ui.separator();
            draw_bottom(self, ctx, ui);
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        clear_app();
    }
}

fn draw_header(_s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {
    egui::menu::bar(_ui, |ui| {
        let is_web = cfg!(target_arch = "wasm32");
        if !is_web {
            ui.menu_button("File", |ui| {
                if ui.button("Quit").clicked() {
                    _ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });

            ui.add_space(16.0);
        }

        egui::widgets::global_dark_light_mode_buttons(ui);
    });
}

fn draw_content(_s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {
    _ui.heading("Nixiv Tools");
    _ui.text_edit_singleline(&mut _s.label);

    if _ui.button("Update").clicked() {
        let parsed_result: Result<u32, _> = _s.label.parse();
        _s.information = draw_item(parsed_result);
    }

    _ui.heading(&_s.information);
}

#[cfg(not(target_arch = "wasm32"))]
fn draw_item(_parsed_result: Result<u32, std::num::ParseIntError>) -> String {
    let res: String;
    match _parsed_result {
        Ok(id) => {
            let rt: Runtime = Runtime::new().unwrap();
            let result: Result<xivapi::models::item::Item, String> = rt.block_on(get_data(id));
            match result {
                Ok(item) => {
                    res = format!(
                        "Item: id = {}, name = {}, hq = {}",
                        item.id, item.name, item.can_be_hq
                    );
                }
                Err(e) => {
                    res = format!("Error fetching item: {}", e);
                }
            }
        }
        Err(e) => {
            res = format!("Error fetching item: {}", e);
        }
    }
    return res;
}

#[cfg(target_arch = "wasm32")]
fn draw_item(_parsed_result: Result<u32, std::num::ParseIntError>) -> String {
    let res: String;
    match _parsed_result {
        Ok(_id) => {
            res = format!("ok");
        }
        Err(e) => {
            res = format!("Error fetching item: {}", e);
        }
    }
    return res;
}

#[cfg(not(target_arch = "wasm32"))]
async fn get_data(_id: u32) -> Result<xivapi::models::item::Item, String> {
    let handle = tokio::spawn(async move {
        update_data(_id).await
    });

    match handle.await {
        Ok(data) => data,
        Err(e) => {
            panic!("Task failed: {:?}", e);
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub async fn get_data(_id: u32) -> Result<xivapi::models::item::Item, String> {
    return Ok(xivapi::models::item::Item::default_item())
}

async fn update_data(_id: u32) -> Result<xivapi::models::item::Item, String> {
    match xivapi::models::item::Item::get_item_from_id(_id).await {
        Ok(item) => Ok(item),
        Err(e) => {
            errlog!("Error: {}", e);
            Err(format!("Error: {}", e))
        }
    }
}

// TODO: Will be implemented
fn draw_bottom(_s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {}

// TODO: Will be implemented
fn clear_app() {}
