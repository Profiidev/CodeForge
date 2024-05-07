use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub(crate) struct Token {
  pub(crate) start: u32,
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
        .split('\n')
        .map(|t| t.replace('\r', ""))
        .collect::<Vec<String>>();

      for i in 0..tokens.len() {
        let start = match self.tokens.last().unwrap().last() {
          Some(t) => t.start + t.token.len() as u32,
          None => 0,
        };
        if token.token != tokens[0] {
          self.tokens.last_mut().unwrap().push(Token {
            start,
            token: tokens[i].clone(),
            type_: token.type_.clone(),
            modifiers: token.modifiers.clone(),
          });

          if i != tokens.len() - 1 {
            self.tokens.push(Vec::new());
          }
        } else {
          self.tokens.last_mut().unwrap().push(Token {
            start,
            token: tokens[i].clone(),
            type_: token.type_.clone(),
            modifiers: token.modifiers.clone(),
          });
        }
      }
    }
  }

  pub(crate) fn merge(&mut self, other: TokenTree) {
    let mut open_brackets = Vec::new();
    let default = Vec::new();
    for (i, tokens) in self.tokens.iter_mut().enumerate() {
      let other_tokens = match other.tokens.get(i) {
        Some(tokens) => tokens,
        None => &default,
      };
      for token in tokens {
        let other_token = other_tokens
          .iter()
          .find(|t| t.start == token.start && t.token == token.token)
          .cloned();

        if let Some(other_token) = other_token {
          token.type_ = other_token.type_;
          token.modifiers = other_token.modifiers;
        }

        match token.token.as_str() {
          "{" | "[" | "(" | "<" => {
            open_brackets.push(token.token.clone());
            token.type_ = format!("punctuation.bracket_{}", (open_brackets.len() - 1) % 3);
          }
          "}" => {
            remove_other_bracket(&mut open_brackets, "{", token);
          }
          "]" => {
            remove_other_bracket(&mut open_brackets, "[", token);
          }
          ")" => {
            remove_other_bracket(&mut open_brackets, "(", token);
          }
          ">" => {
            remove_other_bracket(&mut open_brackets, "<", token);
          }
          _ => {}
        }
      }
    }
  }
}

fn remove_other_bracket(open: &mut Vec<String>, inverse_token: &str, token: &mut Token) {
  while let Some(bracket) = open.pop() {
    if bracket == inverse_token {
      token.type_ = format!("punctuation.bracket_{}", open.len() % 3);
      break;
    }
  }
}
