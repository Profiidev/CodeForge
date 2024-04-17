export type Style = {
  tokenColors: {
    [K in TokenType]: TokenColor;
  };
};

export enum TokenType {
  NONE = "none",
  COMMENT = "comment",
  STRING = "string",
  NUMBER = "number",
  VARIABLE = "variable",
  MEMBER_VARIABLE = "member_variable",
  KEYWORD = "keyword",
  OPERATOR = "operator",
  PUNCTUATION = "punctuation",
  FUNCTION_NAME = "function_name",
  FUNCTION_CALL = "function_call",
  FUNCTION_ARGUMENT = "function_argument",
  SEPARATOR = "separator",
  IMPORT = "import",
  ENTITY_NAME = "entity_name",
  TAG = "tag",
  ANNOTATION = "annotation",
  INVALID = "invalid",
  TYPE = "type",
  BRACKET = "bracket",
  BRACE = "brace",
  PARENTHESIS = "parenthesis",
  ATTRIBUTE = "attribute",
}

export type TokenColor = `#${string}`;
