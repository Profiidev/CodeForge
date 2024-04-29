use std::io::Error;

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use super::{
  parser::{ParsersManager, HIGHLIGHTING_NAMES},
  token::{Token, TokenTree},
};

pub(crate) struct FileManager {
  open_files: Vec<File>,
}

pub(crate) struct File {
  path: String,
  content: String,
  highlighter: Option<(Highlighter, HighlightConfiguration)>,
}

impl FileManager {
  pub(crate) fn new() -> Self {
    FileManager {
      open_files: Vec::new(),
    }
  }

  pub(crate) fn open_file(
    &mut self,
    path: String,
    parser_manager: &ParsersManager,
  ) -> Result<(), Error> {
    let highlighter = match parser_manager.get_language(&path) {
      Some((language, highlights_query, injection_query, locals_query)) => {
        match HighlightConfiguration::new(
          language.clone(),
          "name",
          highlights_query,
          injection_query,
          locals_query,
        ) {
          Ok(mut config) => {
            config.configure(&HIGHLIGHTING_NAMES);
            Some((Highlighter::new(), config))
          }
          Err(err) => {
            eprintln!("Error creating highlight configuration: {}", err);
            None
          }
        }
      }
      None => None,
    };

    let content = std::fs::read_to_string(&path)?;

    self.open_files.push(File {
      path,
      content,
      highlighter,
    });
    Ok(())
  }

  pub(crate) fn close_file(&mut self, path: &str) {
    self.open_files.retain(|file| file.path != path);
  }

  pub(crate) fn get_file(&self, path: &str) -> Option<&File> {
    for file in &self.open_files {
      if file.path == path {
        return Some(file);
      }
    }

    None
  }

  pub(crate) fn get_file_mut(&mut self, path: &str) -> Option<&mut File> {
    for file in &mut self.open_files {
      if file.path == path {
        return Some(file);
      }
    }

    None
  }

  pub(crate) fn get_highlighting(&mut self, path: &str) -> Option<TokenTree> {
    let file = self.get_file_mut(path)?;
    let (highlighter, config) = file.highlighter.as_mut()?;
    let highlights = highlighter
      .highlight(config, file.content.as_bytes(), None, |_| None)
      .ok()?;

    let mut tokens = Vec::new();
    let mut current_token_types: Vec<String> = Vec::new();
    for event in highlights {
      match event.unwrap() {
        HighlightEvent::Source { start, end } => {
          if !current_token_types.is_empty() {
            tokens.push(Token {
              token: file.content[start..end].to_string(),
              type_: current_token_types.last().unwrap().to_string(),
            });
          } else {
            tokens.push(Token {
              token: file.content[start..end].to_string(),
              type_: "".to_string(),
            });
          }
        }
        HighlightEvent::HighlightStart(s) => {
          current_token_types.push(HIGHLIGHTING_NAMES[s.0].to_string());
        }
        HighlightEvent::HighlightEnd => {
          current_token_types.pop();
        }
      }
    }

    let mut token_tree = TokenTree::new();
    token_tree.set_tokens(tokens);

    Some(token_tree)
  }
}
