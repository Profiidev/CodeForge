use lsp_types::SemanticToken;
use serde::Serialize;

use crate::lsp::info::SemanticTokenInfo;

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub(crate) struct Token {
  pub(crate) token: String,
  pub(crate) token_type: String,
  pub(crate) modifiers: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default, Serialize)]
pub(crate) struct TokenTree {
  pub(crate) tokens: Vec<Vec<Token>>
}

impl TokenTree {
  pub(crate) fn new() -> Self {
    Self {
      tokens: Vec::new()
    }
  }

  pub(crate) fn set_tokens(&mut self, base_tokens: Vec<SemanticToken>, token_info: &SemanticTokenInfo, file: &str) {
    self.tokens.push(Vec::new());

    let mut current_char = 0;
    let mut current_line = 0;
    for base_token in base_tokens {
      for _ in 0..base_token.delta_line {
        current_char += file[current_char..].find('\n').unwrap_or(file.len() - 2) + 1;
        current_line += 1;
        self.tokens.push(Vec::new());
      }
      current_char += base_token.delta_start as usize;
      
      let token = Token {
        token: file[current_char..current_char + base_token.length as usize].to_string(),
        token_type: token_info.get_token_type(base_token.token_type),
        modifiers: token_info.get_token_modifier(base_token.token_modifiers_bitset),
      };

      self.tokens[current_line].push(token);
    }
  }
}