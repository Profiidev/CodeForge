use std::io::Error;

use regex::Regex;
use tree_sitter::Language;

pub(crate) struct ParsersManager {
  languages: Vec<ParserLanguage>,
}

pub(crate) struct ParserLanguage {
  language: (Language, String, String, String),
  file_pattern: Regex,
}

impl ParsersManager {
  pub(crate) fn new() -> Self {
    ParsersManager {
      languages: Vec::new(),
    }
  }

  pub(crate) fn add_language(
    &mut self,
    language: Language,
    highlights_query: &str,
    injection_query: &str,
    locals_query: &str,
    file_pattern: String,
  ) -> Result<(), Error> {
    if tree_sitter::LANGUAGE_VERSION != language.version() {
      return Err(Error::new(
        std::io::ErrorKind::InvalidData,
        "Language version mismatch",
      ));
    }

    let language_name = Regex::new(&file_pattern).or_else(|_| {
      Err(Error::new(
        std::io::ErrorKind::InvalidData,
        "Invalid file pattern",
      ))
    })?;

    self.languages.push(ParserLanguage {
      language: (
        language,
        highlights_query.to_string(),
        injection_query.to_string(),
        locals_query.to_string(),
      ),
      file_pattern: language_name,
    });

    Ok(())
  }

  pub(crate) fn get_language(
    &self,
    file_name: &str,
  ) -> Option<&(Language, String, String, String)> {
    for parser_language in &self.languages {
      if parser_language.file_pattern.is_match(file_name) {
        return Some(&parser_language.language);
      }
    }

    None
  }
}
