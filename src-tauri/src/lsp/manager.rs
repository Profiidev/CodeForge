use std::io::Error;

use lsp_types::request::Request as LSPRequestTrait;
use tauri::Url;

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
