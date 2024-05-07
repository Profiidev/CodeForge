use anyhow::Error;
use lsp_types::{
  request::{Request as LSPRequestTrait, SemanticTokensFullRequest},
  SemanticTokensParams, SemanticTokensResult,
};
use tauri::Url;

use crate::file::{parser::get_highlighting_name, token::Token};

use super::{client::LSPData, request::LSPRequest, response::LSPResponse};

#[derive(Debug, Clone)]
pub(crate) struct LSPManager {
  lsps: Vec<LSPData>,
}

impl LSPManager {
  pub(crate) fn new() -> Self {
    LSPManager { lsps: Vec::new() }
  }

  pub(crate) fn add_lsp(&mut self, lsp: LSPData) {
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
    let lsp = self.get_lsp(&path).ok_or(anyhow::anyhow!("no lsp client found for file"))?;

    lsp.send_req(req).await
  }

  fn get_lsp(&self, path: &Url) -> Option<&LSPData> {
    self.lsps.iter().find(|lsp| {
      lsp
        .file_patterns
        .iter()
        .any(|pattern| pattern.is_match(path.as_str()))
    })
  }

  pub(crate) async fn get_semantic_tokens(
    &self,
    path: &str,
    content: &[String],
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

    let token_info = self
      .get_lsp(&uri)?
      .lsp_info
      .as_ref()?
      .semantic_token_info
      .as_ref()?;

    let mut tokens = vec![Vec::new()];
    let mut current_token = 0;
    let mut current_line = 0;
    for token in data {
      for _ in 0..token.delta_line {
        tokens.push(Vec::new());
        current_line += 1;
        current_token = 0;
      }

      let token_type =
        get_highlighting_name(&token_info.get_token_type(token.token_type)).unwrap_or_default();
      let token_modifiers = token_info.get_token_modifiers(token.token_modifiers_bitset);

      current_token += token.delta_start;
      let new_token = Token {
        start: current_token,
        token: content[current_line]
          [current_token as usize..(current_token + token.length) as usize]
          .to_string(),
        type_: token_type,
        modifiers: Some(token_modifiers),
      };

      tokens.last_mut()?.push(new_token);
    }

    Some(tokens)
  }
}
