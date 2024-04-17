<script lang="ts">
  import { TokenType } from "../../types/style";

  let code: {
    tokens: {
      content: string;
      type: TokenType;
    }[];
  }[] = [
    {
      tokens: [
        { content: "import", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "type", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "{", type: TokenType.BRACE },
        { content: " TokenType", type: TokenType.TYPE },
        { content: " }", type: TokenType.BRACE },
        { content: " ", type: TokenType.NONE },
        { content: "from", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: '"../../types/style"', type: TokenType.STRING },
        { content: ";", type: TokenType.SEPARATOR },
      ],
    },
    {
      tokens: [
        { content: "let", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "code", type: TokenType.VARIABLE },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "{", type: TokenType.BRACE },
        { content: " ", type: TokenType.NONE },
        { content: "tokens", type: TokenType.VARIABLE },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "{", type: TokenType.BRACE },
        { content: " ", type: TokenType.NONE },
        { content: "content", type: TokenType.VARIABLE },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "string", type: TokenType.TYPE },
        { content: ",", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "type", type: TokenType.VARIABLE },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "TokenType", type: TokenType.TYPE },
        { content: " ", type: TokenType.NONE },
        { content: "}", type: TokenType.BRACE },
        { content: "[]", type: TokenType.BRACKET },
        { content: " ", type: TokenType.NONE },
        { content: "}", type: TokenType.BRACE },
        { content: "[]", type: TokenType.BRACKET },
        { content: " ", type: TokenType.NONE },
        { content: "=", type: TokenType.OPERATOR },
        { content: " ", type: TokenType.NONE },
        { content: "[", type: TokenType.BRACKET },
        { content: "{", type: TokenType.BRACE },
        { content: " ", type: TokenType.NONE },
        { content: "tokens", type: TokenType.BRACE },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "[", type: TokenType.BRACKET },
        { content: "{", type: TokenType.BRACE },
        { content: " ", type: TokenType.NONE },
        { content: "content", type: TokenType.STRING },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: '"import"', type: TokenType.STRING },
        { content: ",", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: "type", type: TokenType.STRING },
        { content: ":", type: TokenType.SEPARATOR },
        { content: " ", type: TokenType.NONE },
        { content: '"keyword"', type: TokenType.STRING },
        { content: " ", type: TokenType.NONE },
        { content: "}", type: TokenType.BRACE },
        { content: "]", type: TokenType.BRACKET },
        { content: " ", type: TokenType.NONE },
        { content: "}", type: TokenType.BRACE },
        { content: "]", type: TokenType.BRACKET },
        { content: " ", type: TokenType.NONE },
        { content: "}", type: TokenType.BRACE },
        { content: "]", type: TokenType.BRACKET },
      ],
    },
    {
      tokens: [
        { content: "{", type: TokenType.BRACE },
        { content: "#each", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "code", type: TokenType.VARIABLE },
        { content: " ", type: TokenType.NONE },
        { content: "as", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "block", type: TokenType.VARIABLE },
        { content: "}", type: TokenType.BRACE },
      ],
    },
    {
      tokens: [
        { content: "  ", type: TokenType.NONE },
        { content: "<div", type: TokenType.TAG },
        { content: " ", type: TokenType.NONE },
        { content: "class=", type: TokenType.ATTRIBUTE },
        { content: '"block"', type: TokenType.STRING },
        { content: ">", type: TokenType.TAG },
      ],
    },
    {
      tokens: [
        { content: "    ", type: TokenType.NONE },
        { content: "{#each", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "block.tokens", type: TokenType.VARIABLE },
        { content: " ", type: TokenType.NONE },
        { content: "as", type: TokenType.KEYWORD },
        { content: " ", type: TokenType.NONE },
        { content: "token", type: TokenType.VARIABLE },
        { content: "}", type: TokenType.BRACE },
      ],
    },
    {
      tokens: [
        { content: "      ", type: TokenType.NONE },
        { content: "<span", type: TokenType.TAG },
        { content: " ", type: TokenType.NONE },
        { content: "class=", type: TokenType.ATTRIBUTE },
        { content: '"token"', type: TokenType.STRING },
        { content: " ", type: TokenType.NONE },
        { content: "class:type=", type: TokenType.ATTRIBUTE },
        { content: "{token.type}", type: TokenType.VARIABLE },
        { content: ">", type: TokenType.TAG },
        { content: "{token.content}", type: TokenType.VARIABLE },
        { content: "</span>", type: TokenType.TAG },
      ],
    },
    {
      tokens: [
        { content: "    ", type: TokenType.NONE },
        { content: "{/each}", type: TokenType.KEYWORD },
      ],
    },
    {
      tokens: [
        { content: "  ", type: TokenType.NONE },
        { content: "</div>", type: TokenType.TAG },
      ],
    },
    {
      tokens: [{ content: "{/each}", type: TokenType.KEYWORD }],
    },
  ];
</script>

<div class="editor">
  {#each code as block}
    <div class="block">
      <span>
        {#each block.tokens as token}
          <span class="token" class:type={token.type}>{token.content}</span>
        {/each}
      </span>
    </div>
  {/each}
</div>

<style>
  .editor {
    display: flex;
    flex-direction: column;
    background-color: var(--color-background);
    color: var(--color-primary);
    user-select: text;
  }

  .block {
    display: flex;
  }

  .token {
    white-space: pre;
  }
</style>
