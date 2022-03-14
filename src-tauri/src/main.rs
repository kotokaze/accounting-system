#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri_plugin_log::{fern::colors::ColoredLevelConfig, LogTarget, LoggerBuilder};

fn main() {
  let targets = [LogTarget::LogDir, LogTarget::Stdout, LogTarget::Webview];
  let colors = ColoredLevelConfig::default();
  let logger = LoggerBuilder::new()
    .targets(targets)
    .with_colors(colors)
    .build();

  tauri::Builder::default()
    .plugin(logger)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
