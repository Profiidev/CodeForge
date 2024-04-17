// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{io::Write, process::Command, thread::sleep};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    let current_dir = std::env::current_dir().expect("failed to get current directory");

    let mut lsp = Command::new(format!(
        "{}/rust-analyzer-x86_64-pc-windows-msvc.exe",
        current_dir.display()
    ))
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::inherit())
    .spawn()
    .expect("failed to start rust-analyzer");

    let lsp_stdin = lsp.stdin.as_mut().expect("failed to open stdin");
    //let lsp_stdout = lsp.stdout.as_mut().expect("failed to open stdout");

    let init = format!(r#"{{
      "jsonrpc": "2.0",
      "id": 1,
      "method": "initialize",
      "params": {{
        "processId": {},
        "rootUri": "file://{}/",
        "capabilities": {{
          "textDocument": {{
            "completion": {{
              "completionItem": {{
                "snippetSupport": true
              }}
            }}
          }}
        }}
      }}
    }}"#, std::process::id(), current_dir.display().to_string().replace("\\", "/"));

    let header = format!("Content-Length: {}\r\n\r\n", init.len());
    let message = format!("{}{}", header, init);
    println!("{}", message);

    lsp_stdin.write_all(message.as_bytes()).expect("failed to write to stdin");
    println!();

    let init_notify = r#"{"jsonrpc":"2.0","method":"initialized","params":{}}"#;
    let header = format!("Content-Length: {}\r\n\r\n", init_notify.len());
    let message = format!("{}{}", header, init_notify);
    println!("{}", message);

    lsp_stdin.write_all(message.as_bytes()).expect("failed to write to stdin");
    println!();

    let test = format!(r#"{{
      "jsonrpc": "2.0",
      "id": 12,
      "method": "textDocument/completion",
      "params": {{
        "context": {{
          "triggerKind": 1
        }},
        "textDocument": {{
          "uri": "file:///{}/src/main.rs"
        }},
        "position": {{
          "line": 35,
          "character": 21
        }}
      }}
    }}"#, current_dir.display().to_string().replace("\\", "/"));

    let header = format!("Content-Length: {}\r\n\r\n", test.len());
    let message = format!("{}{}", header, test);
    println!("{}", message);

    lsp_stdin.write_all(message.as_bytes()).expect("failed to write to stdin");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
