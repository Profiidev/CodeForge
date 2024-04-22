// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use async_std::sync::Mutex;
use file::token::TokenTree;
use lsp_types::notification::Progress;
use tauri::{async_runtime::block_on, Manager, State, Url};

use crate::lsp::{notification::LSPNotification, capabilities::get_capabilities, manager::LSPManager, client::LSP};

mod lsp;
mod file;

pub(crate) struct AppState(Mutex<LSPManager>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
async fn greet(state: State<'_, AppState>, file: String) -> Result<TokenTree, ()> {
  let files = file.replace("file:///", "");
  let content = std::fs::read_to_string(&files).expect("failed to read file");

  let path = Url::parse(&file).expect("failed to parse file path");
  let manager = state.0.lock().await;
  let token_tree = manager.get_semantic_tokens(path, &content).await.expect("failed to get semantic tokens");
  Ok(token_tree)
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet])
    .manage(AppState(Mutex::new(LSPManager::new())))
    .setup(|app| {
      let lsp = block_on(test_lsp());
      let state: State<'_, AppState> = app.state();
      block_on(async {
        state.0.lock().await.add_lsp(lsp)
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
      println!("{:?}", params);
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

  let lsp = lsp::LSP::create(format!(
    "{}/lsp_test/rust-analyzer-x86_64-pc-windows-msvc.exe",
    current_dir.display()
  ), vec!["^.+\\.rs$".to_string()])
  .expect("failed to start rust-analyzer")
  .build(work_dir, get_capabilities(), not_handler)
  .await
  .expect("failed to build lsp client");

  lsp
}