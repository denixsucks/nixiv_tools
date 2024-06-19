use crate::xivapi::models::item::Item;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NixivApp {
  label: String,
  item_info: String,

  #[serde(skip)]
  value: f32,
}

impl Default for NixivApp {
  fn default() -> Self {
    Self {
      label: "".to_owned(),
      item_info: "".to_owned(),
      value: 2.7,
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
      draw_center(self, ctx, ui);
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

fn draw_center(_s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {
  _ui.heading("Nixiv Tools");
  _ui.text_edit_singleline(&mut _s.label);

  if _ui.button("Update").clicked() {
    let parsed_result: Result<u32, _> = _s.label.parse();
    match update_item_data(parsed_result, _ui) {
      Ok(item) => {
        _s.item_info = format!("Item: id = {}, name = {}, hq = {}", item.id, item.name, item.can_be_hq);
      },
      Err(e) => {
        _s.item_info = format!("Failed to retrieve item: {}", e);
      }
    }
  }

  _ui.heading(&_s.item_info);
}

fn update_item_data<E: std::fmt::Debug>(parsed_result: Result<u32, E>, _ui: &mut egui::Ui) -> Result<Item, String> {
  let e : String = "Not implemented yet.".to_owned();
  Err(e)
}

fn draw_bottom(_s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {
}

fn clear_app() {
}