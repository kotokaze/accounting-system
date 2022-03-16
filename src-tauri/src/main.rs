#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};
use tauri_plugin_store::{PluginBuilder, StoreBuilder};

fn main() {
  let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
  let colors = ColoredLevelConfig::default();
  let logger = LoggerBuilder::new()
    .targets(targets)
    .with_colors(colors)
    .build();

  let store = PluginBuilder::default();
  let settings = StoreBuilder::new("store.bin".parse().unwrap())
    .default("key".to_string(), "hello world".into())
    .build();

  tauri::Builder::default()
    .plugin(logger)
    .plugin(store.store(settings).freeze().build())
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
