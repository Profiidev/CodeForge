use anyhow::Error;
use regex::Regex;
use tree_sitter::Language;

pub(crate) const HIGHLIGHTING_NAMES: [&str; 46] = [
  "variable",
  "variable.builtin",
  "variable.parameter",
  "variable.member",
  "constant",
  "constant.builtin",
  "module",
  "label",
  "string",
  "string.documentation",
  "string.escape",
  "string.regexp",
  "character",
  "boolean",
  "number",
  "number.float",
  "type",
  "type.builtin",
  "type.definition",
  "attribute",
  "attribute.builtin",
  "function",
  "function.builtin",
  "function.macro",
  "function.call",
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
  "keyword.directive",
  "punctuation.delimiter",
  "punctuation.bracket",
  "punctuation.special",
  "punctuation.accessor",
  "comment",
  "comment.documentation",
];

pub(crate) const SEMANTIC_TOKEN_MAP: [(&str, &str); 27] = [
  ("namespace", "module"),
  ("type", "type"),
  ("class", "type"),
  ("enum", "type"),
  ("interface", "type"),
  ("struct", "type"),
  ("typeParameter", "type"),
  ("parameter", "variable.parameter"),
  ("variable", "variable"),
  ("property", "variable.member"),
  ("enumMember", "type"),
  ("event", "variable"),
  ("function", "function"),
  ("method", "function"),
  ("macro", "function.macro"),
  ("keyword", "keyword"),
  ("modifier", "keyword.modifier"),
  ("comment", "comment"),
  ("string", "string"),
  ("number", "number"),
  ("regexp", "string.regexp"),
  ("operator", "operator"),
  ("decorator", "label"),
  ("punctuation", "punctuation.delimiter"),
  ("brace", "punctuation.bracket"),
  ("bracket", "punctuation.bracket"),
  ("parenthesis", "punctuation.bracket"),
];

pub(crate) fn get_highlighting_name(name: &str) -> Option<String> {
  SEMANTIC_TOKEN_MAP.iter().find_map(|(key, value)| {
    if key == &name {
      Some(value.to_string())
    } else {
      None
    }
  })
}

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
      return Err(anyhow::anyhow!("Language version mismatch"));
    }

    let language_name = Regex::new(&file_pattern)?;

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
