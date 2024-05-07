use anyhow::Error;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use crate::lsp::manager::LSPManager;

use super::{
  parser::{ParsersManager, HIGHLIGHTING_NAMES},
  token::{Token, TokenTree},
};

pub(crate) struct FileManager {
  open_files: Vec<File>,
}

pub(crate) struct File {
  path: String,
  content: Vec<String>,
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
    path: &str,
    parser_manager: &ParsersManager,
  ) -> Result<(), Error> {
    let highlighter = match parser_manager.get_language(path) {
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

    //read an add \r\n to each line
    let content = std::fs::read_to_string(path)?
      .lines()
      .map(|s| s.to_string())
      .map(|s| s + "\r\n")
      .collect();

    self.open_files.push(File {
      path: path.to_string(),
      content,
      highlighter,
    });
    Ok(())
  }

  pub(crate) fn close_file(&mut self, path: &str) {
    self.open_files.retain(|file| file.path != path);
  }

  pub(crate) fn get_file(&self, path: &str) -> Option<&File> {
    self.open_files.iter().find(|file| file.path == path)
  }

  pub(crate) fn get_file_mut(&mut self, path: &str) -> Option<&mut File> {
    self.open_files.iter_mut().find(|file| file.path == path)
  }

  pub(crate) fn get_highlighting(&mut self, path: &str) -> Option<TokenTree> {
    let file = self.get_file_mut(path)?;
    let content = file.content.join("");
    let (highlighter, config) = file.highlighter.as_mut()?;
    let highlights = highlighter
      .highlight(config, content.as_bytes(), None, |_| None)
      .ok()?;

    let mut tokens = Vec::new();
    let mut current_token_types: Vec<String> = Vec::new();
    for event in highlights {
      match event.unwrap() {
        HighlightEvent::Source { start, end } => {
          if !current_token_types.is_empty() {
            tokens.push(Token {
              start: 0,
              token: content[start..end].to_string(),
              type_: current_token_types.last().unwrap().to_string(),
              modifiers: None,
            });
          } else {
            tokens.push(Token {
              start: 0,
              token: content[start..end].to_string(),
              type_: "".to_string(),
              modifiers: None,
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
    token_tree.set_raw_tokens(tokens);

    Some(token_tree)
  }

  pub(crate) async fn get_semantic_highlighting(
    &mut self,
    path: &str,
    lsp_manager: &LSPManager,
  ) -> Option<TokenTree> {
    let mut syntax_tree = self.get_highlighting(path)?;
    let file = self.get_file_mut(path)?;

    let mut semantic_tree = TokenTree::new();
    semantic_tree.set_tokens(
      lsp_manager
        .get_semantic_tokens(path, &file.content)
        .await?,
    );

    syntax_tree.merge(semantic_tree);

    Some(syntax_tree)
  }
}
