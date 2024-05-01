use std::io::Error;

use lsp_types::{
  request::{Request as LSPRequestTrait, SemanticTokensFullRequest},
  SemanticTokensParams, SemanticTokensResult,
};
use tauri::Url;

use crate::file::{parser::get_highlighting_name, token::Token};

use super::{client::LSP, request::LSPRequest, response::LSPResponse};

#[derive(Debug, Clone)]
pub(crate) struct LSPManager {
  lsps: Vec<LSP>,
}

impl LSPManager {
  pub(crate) fn new() -> Self {
    LSPManager { lsps: Vec::new() }
  }

  pub(crate) fn add_lsp(&mut self, lsp: LSP) {
    self.lsps.push(lsp);
  }

  pub(crate) async fn send_req<T>(
    &self,
    req: LSPRequest<T>,
    path: Url,
  ) -> Result<Option<LSPResponse<T>>, Error>
  where
    T: LSPRequestTrait,
  {
    let lsp = self.get_lsp(&path).ok_or(Error::new(
      std::io::ErrorKind::InvalidInput,
      "no lsp client found for file",
    ))?;

    lsp.send_req(req).await
  }

  fn get_lsp(&self, path: &Url) -> Option<&LSP> {
    self.lsps.iter().find(|lsp| {
      lsp
        .file_patterns
        .iter()
        .any(|pattern| pattern.is_match(&path.as_str()))
    })
  }

  pub(crate) async fn get_semantic_tokens(
    &self,
    path: &str,
    content: String,
  ) -> Option<Vec<Vec<Token>>> {
    let uri = Url::parse(&format!("file://{}", path)).ok()?;
    let test = LSPRequest::<SemanticTokensFullRequest>::new(Some(SemanticTokensParams {
      text_document: lsp_types::TextDocumentIdentifier { uri: uri.clone() },
      partial_result_params: Default::default(),
      work_done_progress_params: Default::default(),
    }));

    let res = self
      .send_req(test, uri.clone())
      .await
      .ok()??
      .result
      .ok()??;

    let data = match res {
      SemanticTokensResult::Partial(partial) => partial.data,
      SemanticTokensResult::Tokens(full) => full.data,
    };
    println!("{:?}", data);
    let token_info = self
      .get_lsp(&uri)?
      .lsp_info
      .as_ref()?
      .semantic_token_info
      .as_ref()?;

    let mut tokens = vec![Vec::new()];
    for token in data {
      //TODO: handle tokens
    }

    Some(tokens)
  }
}
