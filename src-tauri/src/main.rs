// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::sync::Mutex;
use file::{manager::FileManager, parser::ParsersManager};
use lsp_types::notification::Progress;
use tauri::{async_runtime::block_on, Manager, State, Url};

use crate::lsp::{
  capabilities::get_capabilities, client::LSP, manager::LSPManager, notification::LSPNotification,
};

mod file;
mod lsp;

pub(crate) struct AppState(Mutex<LSPManager>, Mutex<FileManager>, Mutex<ParsersManager>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(state: State<'_, AppState>) -> Result<String, ()> {
  let start = std::time::Instant::now();
  let mut res = state
    .1
    .lock()
    .await;
  let ress = res.get_highlighting("src/main.rs");
  println!("{:?}", ress);
  println!("Highlighting done in {:?}", start.elapsed());
  Ok("Hello".to_string())
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .manage(AppState(
      Mutex::new(LSPManager::new()),
      Mutex::new(FileManager::new()),
      Mutex::new(ParsersManager::new()),
    ))
    .setup(|app| {
      let lsp = block_on(test_lsp());
      let state: State<'_, AppState> = app.state();

      block_on(async {
        state.0.lock().await.add_lsp(lsp);
        state
          .2
          .lock()
          .await
          .add_language(
            tree_sitter_rust::language(),
            tree_sitter_rust::HIGHLIGHT_QUERY,
            tree_sitter_rust::INJECTIONS_QUERY,
            tree_sitter_rust::TAGS_QUERY,
            "^.+\\.rs$".to_string(),
          )
          .unwrap();
        state
          .1
          .lock()
          .await
          .open_file("src/main.rs".to_string(), &*state.2.lock().await)
          .unwrap();
      });

      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

fn not_handler(not: lsp::notification::RawLSPNotification) {
  match not.method.as_str() {
    "$/progress" => {
      let params: LSPNotification<Progress> = not.parse().unwrap();
      //println!("{:?}", params);
    }
    _ => {}
  }
}

async fn test_lsp() -> LSP {
  let current_dir = std::env::current_dir().expect("failed to get current directory");

  let work_dir = vec![lsp_types::WorkspaceFolder {
    uri: Url::from_directory_path(current_dir.clone()).expect("failed to convert path to url"),
    name: current_dir
      .file_name()
      .expect("failed to get file name")
      .to_string_lossy()
      .to_string(),
  }];

  let lsp = lsp::LSP::create(
    format!(
      "{}/lsp_test/rust-analyzer-x86_64-pc-windows-msvc.exe",
      current_dir.display()
    ),
    vec!["^.+\\.rs$".to_string()],
  )
  .expect("failed to start rust-analyzer")
  .build(work_dir, get_capabilities(), not_handler)
  .await
  .expect("failed to build lsp client");

  lsp
}
