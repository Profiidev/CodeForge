use std::io::Error;

use lsp_types::{request::Initialize, SemanticTokensOptions, SemanticTokensServerCapabilities};

use super::response::LSPResponse;

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct LSPInfo {
  pub(crate) semantic_token_info: Option<SemanticTokenInfo>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub(crate) struct SemanticTokenInfo {
  token_types: Vec<String>,
  token_modifiers: Vec<String>,
}

impl LSPInfo {
  pub(crate) fn new(init_res: LSPResponse<Initialize>) -> Result<Self, Error> {
    let server_capabilities = init_res.result.expect("no result").capabilities;
    let semantic_token_info = match server_capabilities.semantic_tokens_provider {
      Some(token_capability) => match token_capability {
        SemanticTokensServerCapabilities::SemanticTokensOptions(options) => {
          Some(SemanticTokenInfo::new(options))
        }
        SemanticTokensServerCapabilities::SemanticTokensRegistrationOptions(reg) => {
          Some(SemanticTokenInfo::new(reg.semantic_tokens_options))
        }
      },
      None => None,
    };

    Ok(Self {
      semantic_token_info,
    })
  }
}

impl SemanticTokenInfo {
  pub(crate) fn new(options: SemanticTokensOptions) -> Self {
    Self {
      token_types: options
        .legend
        .token_types
        .iter()
        .map(|s| s.as_str().to_string())
        .collect(),
      token_modifiers: options
        .legend
        .token_modifiers
        .iter()
        .map(|s| s.as_str().to_string())
        .collect(),
    }
  }

  pub(crate) fn get_token_type(&self, idx: u32) -> String {
    self.token_types[idx as usize].clone()
  }

  pub(crate) fn get_token_modifiers(&self, modifiers: u32) -> Vec<String> {
    let mut modifiers_vec = Vec::new();
    for i in 0..31 {
      if modifiers & (1 << i) != 0 {
        modifiers_vec.push(self.token_modifiers[i].clone());
      }
    }
    modifiers_vec
  }
}
