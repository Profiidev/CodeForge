use std::fs;

use tree_sitter::Parser;
use tree_sitter_highlight::{HighlightConfiguration, HighlightEvent, Highlighter};

static NAMES: [&str; 57] = [
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
];

#[derive(Debug)]
struct Token {
  token: String,
  type_: String,
}

fn main() {
  let mut highlighter = Highlighter::new();
  let mut config = HighlightConfiguration::new(
    tree_sitter_configs::language(),
    "name".to_string(),
    tree_sitter_configs::HIGHLIGHTS_QUERY,
    tree_sitter_configs::INJECTIONS_QUERY,
    tree_sitter_configs::TAGS_QUERY,
  )
  .unwrap();

  config.configure(&NAMES);

  let source = fs::read_to_string("src/main.rs").unwrap();

  let highlights = highlighter.highlight(&config, source.as_bytes(), None, |_| None).unwrap();

  let mut tokens = Vec::new();
  let mut current_token_types: Vec<String> = Vec::new();
  for event in highlights {
    match event.unwrap() {
      HighlightEvent::Source { start, end } => {
        if !current_token_types.is_empty() {
          tokens.push(Token {
            token: source[start..end].to_string(),
            type_: current_token_types.last().unwrap().to_string(),
          });
        } else {
          tokens.push(Token {
            token: source[start..end].to_string(),
            type_: "".to_string(),
          });
        }
      }
      HighlightEvent::HighlightStart(s) => {
        current_token_types.push(NAMES[s.0].to_string());
      }
      HighlightEvent::HighlightEnd => {
        current_token_types.pop();
      }
    }
  }

  let mut lines = Vec::new();
  let max_spacing = tokens.iter().map(|token| token.token.len()).max().unwrap();
  for token in tokens {
    lines.push(format!(
      "{:<width$} | {}",
      token.token,
      token.type_,
      width = max_spacing
    ));
  }
  
  fs::write("data/tokens.txt", lines.join("\n")).unwrap();




  let mut parser = Parser::new();
  parser.set_language(&tree_sitter_configs::language()).unwrap();

  let tree = parser.parse(source.as_bytes(), None).unwrap();

  let root = tree.root_node();
  let mut lines = Vec::new();
  let mut current_indent = 0;

  fn print_tree(node: &tree_sitter::Node, lines: &mut Vec<String>, current_indent: &mut usize) {
    let line = format!("{:indent$}{}", "", node.kind(), indent = *current_indent);
    lines.push(line);
    *current_indent += 2;
    for i in 0..node.child_count() {
      print_tree(&node.child(i).unwrap(), lines, current_indent);
    }
    *current_indent -= 2;
  }

  print_tree(&root, &mut lines, &mut current_indent);

  fs::write("data/tree.txt", lines.join("\n")).unwrap();
  
  
  let mut lines = tree.root_node().to_sexp().split("(").filter(|&s| s != "").map(|line| line.to_string()).collect::<Vec<String>>();

  let mut current_indent = 0;
  lines.iter_mut().for_each(|line| {
    let new_line = format!("({}", line);
    *line = format!("{:indent$}{}", "", new_line, indent = current_indent);
    current_indent += 2;
    current_indent -= line.matches(")").count() * 2;
  });

  fs::write("data/sexp.txt", lines.join("\n")).unwrap();
}
