use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Token {
  pub(crate) token: String,
  #[serde(rename = "type")]
  pub(crate) type_: String,
  pub(crate) modifiers: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct TokenTree {
  pub(crate) tokens: Vec<Vec<Token>>,
}

impl TokenTree {
  pub(crate) fn new() -> Self {
    TokenTree { tokens: Vec::new() }
  }

  pub(crate) fn set_tokens(&mut self, tokens: Vec<Vec<Token>>) {
    self.tokens = tokens;
  }

  pub(crate) fn set_raw_tokens(&mut self, tokens: Vec<Token>) {
    self.tokens = vec![Vec::new()];
    for token in tokens {
      let tokens = token
        .token
        .split("\n")
        .map(|t| t.replace("\r", ""))
        .collect::<Vec<String>>();
      for i in 0..tokens.len() {
        if token.token != tokens[0] {
          self.tokens.last_mut().unwrap().push(Token {
            token: tokens[i].clone(),
            type_: token.type_.clone(),
            modifiers: token.modifiers.clone(),
          });

          if i != tokens.len() - 1 {
            self.tokens.push(Vec::new());
          }
        } else {
          self.tokens.last_mut().unwrap().push(Token {
            token: tokens[i].clone(),
            type_: token.type_.clone(),
            modifiers: token.modifiers.clone(),
          });
        }
      }
    }
  }
}
