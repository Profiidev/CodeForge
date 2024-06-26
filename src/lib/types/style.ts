export type Style = {
  tokenColors: {
    [K in TokenType]: TokenColor;
  };
};

export enum TokenType {
  Variable = "variable",
  VariableBuiltin = "variable.builtin",
  VariableParameter = "variable.parameter",
  VariableMember = "variable.member",
  Constant = "constant",
  ConstantBuiltin = "constant.builtin",
  Module = "module",
  Label = "label",
  String = "string",
  StringDocumentation = "string.documentation",
  StringEscape = "string.escape",
  StringRegexp = "string.regexp",
  Character = "character",
  Boolean = "boolean",
  Number = "number",
  NumberFloat = "number.float",
  Type = "type",
  TypeBuiltin = "type.builtin",
  TypeDefinition = "type.definition",
  Attribute = "attribute",
  AttributeBuiltin = "attribute.builtin",
  Function = "function",
  FunctionBuiltin = "function.builtin",
  FunctionMacro = "function.macro",
  FunctionCall = "function.call",
  Constructor = "constructor",
  Operator = "operator",
  Keyword = "keyword",
  KeywordCoroutine = "keyword.coroutine",
  KeywordFunction = "keyword.function",
  KeywordOperator = "keyword.operator",
  KeywordImport = "keyword.import",
  KeywordType = "keyword.type",
  KeywordModifier = "keyword.modifier",
  KeywordRepeat = "keyword.repeat",
  KeywordReturn = "keyword.return",
  KeywordDebug = "keyword.debug",
  KeywordException = "keyword.exception",
  KeywordConditional = "keyword.conditional",
  KeywordDirective = "keyword.directive",
  PunctuationDelimiter = "punctuation.delimiter",
  PunctuationBracket = "punctuation.bracket",
  PunctuationBracket0 = "punctuation.bracket_0",
  PunctuationBracket1 = "punctuation.bracket_1",
  PunctuationBracket2 = "punctuation.bracket_2",
  PunctuationSpecial = "punctuation.special",
  PunctuationAccessor = "punctuation.accessor",
  Comment = "comment",
  CommentDocumentation = "comment.documentation",
}

export type TokenColor = `#${string}`;

export let test_style: Style = {
  tokenColors: {
    [TokenType.Variable]: "#bfbdb6",
    [TokenType.VariableBuiltin]: "#ff8f40",
    [TokenType.VariableParameter]: "#d2a6ff",
    [TokenType.VariableMember]: "#bfbdb6",
    [TokenType.Constant]: "#95e6cb",
    [TokenType.ConstantBuiltin]: "#d2a6ff",
    [TokenType.Module]: "#59c2ff",
    [TokenType.Label]: "#ffb454",
    [TokenType.String]: "#aad94c",
    [TokenType.StringDocumentation]: "#aad94c",
    [TokenType.StringEscape]: "#95e6cb",
    [TokenType.StringRegexp]: "#95e6cb",
    [TokenType.Character]: "#aad94c",
    [TokenType.Boolean]: "#d2a6ff",
    [TokenType.Number]: "#d2a6ff",
    [TokenType.NumberFloat]: "#d2a6ff",
    [TokenType.Type]: "#59c2ff",
    [TokenType.TypeBuiltin]: "#59c2ff",
    [TokenType.TypeDefinition]: "#59c2ff",
    [TokenType.Attribute]: "#ff8f40",
    [TokenType.AttributeBuiltin]: "#ff8f40",
    [TokenType.Function]: "#ffb454",
    [TokenType.FunctionBuiltin]: "#ffb454",
    [TokenType.FunctionMacro]: "#ffb454",
    [TokenType.FunctionCall]: "#ffb454",
    [TokenType.Constructor]: "#59c2ff",
    [TokenType.Operator]: "#ff8f40",
    [TokenType.Keyword]: "#ff8f40",
    [TokenType.KeywordCoroutine]: "#ff8f40",
    [TokenType.KeywordFunction]: "#ff8f40",
    [TokenType.KeywordOperator]: "#f29668",
    [TokenType.KeywordImport]: "#ff8f40",
    [TokenType.KeywordType]: "#ff8f40",
    [TokenType.KeywordModifier]: "#ff8f40",
    [TokenType.KeywordRepeat]: "#ff8f40",
    [TokenType.KeywordReturn]: "#ff8f40",
    [TokenType.KeywordDebug]: "#ff8f40",
    [TokenType.KeywordException]: "#ff8f40",
    [TokenType.KeywordConditional]: "#ff8f40",
    [TokenType.KeywordDirective]: "#ff8f40",
    [TokenType.PunctuationDelimiter]: "#bfbdb6b3",
    [TokenType.PunctuationBracket]: "#ff8f40",
    [TokenType.PunctuationBracket0]: "#fcca03",
    [TokenType.PunctuationBracket1]: "#ff00e6",
    [TokenType.PunctuationBracket2]: "#0091ff",
    [TokenType.PunctuationSpecial]: "#ff8f40",
    [TokenType.PunctuationAccessor]: "#ff8f40",
    [TokenType.Comment]: "#acb6bf8c",
    [TokenType.CommentDocumentation]: "#acb6bf8c",
  },
};
