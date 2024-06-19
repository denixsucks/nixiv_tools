use crate::xiv::item::Item;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct NixivApp {
  label: String,

  #[serde(skip)]
  value: f32,
}

impl Default for NixivApp {
  fn default() -> Self {
    Self {
      label: "Hello World!".to_owned(),
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
  if _ui.button("update").clicked() {
    let parsed_result: Result<u32, _> = _s.label.parse();
    update_button(parsed_result, _ui);
  }
}

fn draw_bottom(s: &mut NixivApp, _ctx: &egui::Context, _ui: &mut egui::Ui) {

}


fn update_button<E: std::fmt::Debug>(parsed_result: Result<u32, E>, _ui: &mut egui::Ui) {
  match parsed_result {
    Ok(item_id) => {
      match Item::get_item_from_id(item_id) {
        Ok(item) => {
          _ui.heading(format!("Item: id = {}, name = {}", item.id, item.name));
        },
        Err(e) => {
          eprintln!("Error: {}", e);
        },
      }
    },
    Err(e) => {
      eprintln!("Error parsing string to unsigned integer: {:?}", e);
    },
  }
}

fn clear_app()
{

}
