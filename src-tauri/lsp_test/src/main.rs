use std::thread::sleep;

use lsp::request::LSPRequest;
use lsp::LSPClient;
use lsp_types::Url;

mod lsp;

#[async_std::main]
async fn main() {
  let current_dir = std::env::current_dir().expect("failed to get current directory");

  let cap = lsp_types::ClientCapabilities {
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
  };

  let work_dir = vec![lsp_types::WorkspaceFolder {
    uri: Url::from_directory_path(current_dir.clone()).expect("failed to convert path to url"),
    name: current_dir
      .file_name()
      .expect("failed to get file name")
      .to_string_lossy()
      .to_string(),
  }];

  let lsp = LSPClient::create(format!(
    "{}/rust-analyzer-x86_64-pc-windows-msvc.exe",
    current_dir.display()
  ))
  .expect("failed to start rust-analyzer")
  .build(work_dir, cap)
  .await
  .expect("failed to build lsp client");

  println!("lsp client started");

  let comp: LSPRequest<lsp_types::request::Completion> =
    LSPRequest::new(Some(lsp_types::CompletionParams {
      text_document_position: lsp_types::TextDocumentPositionParams {
        text_document: lsp_types::TextDocumentIdentifier {
          uri: Url::from_directory_path(current_dir.join("src/main.rs"))
            .expect("failed to convert path to url"),
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
    }))
    .unwrap();

  let res = lsp.send_req(comp).await.expect("failed to send request");
  println!("{:?}", res);

  sleep(std::time::Duration::from_secs(10000));
}
