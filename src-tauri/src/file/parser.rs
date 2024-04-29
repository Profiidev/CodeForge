use std::io::Error;

use regex::Regex;
use tree_sitter::Language;

pub(crate) const HIGHLIGHTING_NAMES: [&str; 94] = [
  "variable",
  "variable.builtin",
  "variable.parameter",
  "variable.parameter.builtin",
  "variable.member",
  "constant",
  "constant.builtin",
  "constant.macro",
  "module",
  "module.builtin",
  "label",
  "string",
  "string.documentation",
  "string.escape",
  "string.regexp",
  "string.special",
  "string.special.url",
  "string.special.path",
  "string.special.symbol",
  "character",
  "character.special",
  "boolean",
  "number",
  "number.float",
  "type",
  "type.builtin",
  "type.definition",
  "attribute",
  "attribute.builtin",
  "property",
  "function",
  "function.builtin",
  "function.macro",
  "function.call",
  "function.method",
  "function.method.call",
  "constructor",
  "operator",
  "keyword",
  "keyword.coroutine",
  "keyword.function",
  "keyword.operator",
  "keyword.import",
  "keyword.type",
  "keyword.modifier",
  "keyword.repeat",
  "keyword.return",
  "keyword.debug",
  "keyword.exception",
  "keyword.conditional",
  "keyword.conditional.ternary",
  "keyword.directive",
  "keyword.directive.define",
  "punctuation.delimiter",
  "punctuation.bracket",
  "punctuation.special",
  "comment",
  "comment.documentation",
  "comment.error",
  "comment.todo",
  "comment.warning",
  "comment.note",
  "markup.strong",
  "markup.italic",
  "markup.underline",
  "markup.strikethrough",
  "markup.link",
  "markup.link.url",
  "markup.link.label",
  "markup.quote",
  "markup.math",
  "markup.heading",
  "markup.heading.1",
  "markup.heading.2",
  "markup.heading.3",
  "markup.heading.4",
  "markup.heading.5",
  "markup.heading.6",
  "markup.list",
  "markup.list.checked",
  "markup.list.unchecked",
  "markup.raw",
  "markup.raw.block",
  "diff.plus",
  "diff.minus",
  "diff.delta",
  "tag",
  "tag.attribute",
  "tag.builtin",
  "tag.delimiter",
  "spell",
  "nospell",
  "none",
  "conceal",
];

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
