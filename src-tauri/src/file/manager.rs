use std::io::Error;

use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

use super::parser::ParsersManager;

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
            let highlight_names = [
              "attribute",
              "constant",
              "function.builtin",
              "function",
              "keyword",
              "operator",
              "property",
              "punctuation",
              "punctuation.bracket",
              "punctuation.delimiter",
              "string",
              "string.special",
              "tag",
              "type",
              "type.builtin",
              "variable",
              "variable.builtin",
              "variable.parameter",
            ];
            config.configure(&highlight_names);
            Some((Highlighter::new(), config))
          }
          Err(err) => {
            eprintln!("Error creating highlight configuration: {}", err);
            None
          },
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

  pub(crate) fn get_highlighting(&mut self, path: &str) -> Option<&str> {
    let file = self.get_file_mut(path)?;
    let (highlighter, config) = file.highlighter.as_mut()?;
    let highlights = highlighter
      .highlight(config, file.content.as_bytes(), None, |_| None)
      .ok()?;
    for event in highlights {
      match event.unwrap() {
        HighlightEvent::Source { start, end } => {
          println!("source: {}-{}", start, end);
        }
        HighlightEvent::HighlightStart(s) => {
          println!("highlight style started: {:?}", s);
        }
        HighlightEvent::HighlightEnd => {
          println!("highlight style ended");
        }
      }
    }

    None
  }
}
