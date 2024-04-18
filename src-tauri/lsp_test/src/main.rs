use std::{io::Write, process::Command, thread::sleep, vec};

use json_rpc_types::Request;
use lsp_types::{notification::{Initialized, Notification}, request::{Completion, Initialize, Request as LSPRequest}, CompletionParams, InitializeParams, Url};

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

    let params = InitializeParams {
        process_id: Some(std::process::id() as u32),
        workspace_folders: Some(vec![lsp_types::WorkspaceFolder {
            uri: Url::from_directory_path(current_dir.clone()).expect("failed to convert path to url"),
            name: current_dir.file_name().expect("failed to get file name").to_string_lossy().to_string()
        }]),
        capabilities: lsp_types::ClientCapabilities {
            text_document: Some(lsp_types::TextDocumentClientCapabilities {
                completion: Some(lsp_types::CompletionClientCapabilities {
                    completion_item: Some(lsp_types::CompletionItemCapability {
                        snippet_support: Some(true),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        },
        ..Default::default()
    };
    let init = Request {
        jsonrpc: json_rpc_types::Version::V2,
        id: Some(json_rpc_types::Id::Num(1)),
        method: Initialize::METHOD.to_string(),
        params: Some(params),
    };

    let inited = Request {
        jsonrpc: json_rpc_types::Version::V2,
        method: Initialized::METHOD.to_string(),
        params: Some(()),
        id: None,
    };

    let params = CompletionParams {
        text_document_position: lsp_types::TextDocumentPositionParams {
            text_document: lsp_types::TextDocumentIdentifier {
                uri: Url::from_directory_path(current_dir.join("src/main.rs")).expect("failed to convert path to url"),
            },
            position: lsp_types::Position {
                line: 36,
                character: 11,
            },
        },
        context: Some(lsp_types::CompletionContext {
            trigger_kind: lsp_types::CompletionTriggerKind::INVOKED,
            trigger_character: None,
        }),
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
    };

    let completion = Request {
        jsonrpc: json_rpc_types::Version::V2,
        id: Some(json_rpc_types::Id::Num(12)),
        method: Completion::METHOD.to_string(),
        params: Some(params),
    };

    let init = serde_json::to_string(&init).expect("failed to serialize request");
    let inited = serde_json::to_string(&inited).expect("failed to serialize request");
    let completion = serde_json::to_string(&completion).expect("failed to serialize request");

    let init = format!("Content-Length: {}\r\n\r\n{}", init.len(), init);
    let inited = format!("Content-Length: {}\r\n\r\n{}", inited.len(), inited);
    let completion = format!("Content-Length: {}\r\n\r\n{}", completion.len(), completion);

    lsp_stdin.write_all(init.as_bytes()).expect("failed to write to stdin");
    lsp_stdin.write_all(inited.as_bytes()).expect("failed to write to stdin");
    sleep(std::time::Duration::from_secs(5));
    lsp_stdin.write_all(completion.as_bytes()).expect("failed to write to stdin");

    sleep(std::time::Duration::from_secs(20));
}
