#![warn(clippy::all, rust_2018_idioms)]

mod app;
pub use app::NixivApp;
pub use xivapi;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;
#[cfg(target_arch = "wasm32")]
use web_sys::console;

#[macro_export]
macro_rules! log {
  ($($t:tt)*) => {
    #[cfg(target_arch = "wasm32")] {
      use web_sys::console;
      console::log_1(&format!($($t)*).into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    println!($($t)*);
  }
}

#[macro_export]
macro_rules! errlog {
  ($($t:tt)*) => {
    #[cfg(target_arch = "wasm32")] {
      use web_sys::console;
      console::error_1(&format!($($t)*).into());
    }

    #[cfg(not(target_arch = "wasm32"))]
    eprintln!($($t)*);
  }
}
