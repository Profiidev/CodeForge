use std::io::Error;

use lsp_types::{
  request::{Request as LSPRequestTrait, SemanticTokensFullRequest},
  SemanticTokensParams, SemanticTokensResult,
};
use tauri::Url;

use crate::file::token::TokenTree;

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

  pub(crate) async fn get_semantic_tokens(&self, path: Url, content: &str) -> Result<TokenTree, Error> {
    let req = LSPRequest::<SemanticTokensFullRequest>::new(Some(SemanticTokensParams {
      work_done_progress_params: Default::default(),
      partial_result_params: Default::default(),
      text_document: lsp_types::TextDocumentIdentifier { uri: path.clone() },
    }))?;

    let res = self.send_req(req, path.clone()).await?;
    let res = res
      .ok_or(Error::new(std::io::ErrorKind::InvalidData, "no response"))?
      .result;
    let tokens = match res {
      Ok(tokens) => tokens.ok_or(Error::new(std::io::ErrorKind::InvalidData, "no tokens"))?,
      Err(err) => return Err(Error::new(std::io::ErrorKind::InvalidData, err.message)),
    };
    let tokens = match tokens {
      SemanticTokensResult::Tokens(tokens) => tokens.data,
      SemanticTokensResult::Partial(tokens) => tokens.data,
    };

    let info = self.get_lsp(&path).unwrap().clone().lsp_info.ok_or(Error::new(
      std::io::ErrorKind::InvalidInput,
      "no lsp info found for file",
    ))?.semantic_token_info.ok_or(Error::new(
      std::io::ErrorKind::InvalidInput,
      "no semantic token info found for file",
    ))?;

    let mut token_tree = TokenTree::new();
    token_tree.set_tokens(tokens, &info, content);

    Ok(token_tree)
  }

  async fn send_req<T>(
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
}
