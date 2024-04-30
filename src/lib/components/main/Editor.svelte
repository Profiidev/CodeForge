<script lang="ts">
  import { TokenType, test_style } from "../../types/style";
  import { invoke } from "@tauri-apps/api/core";

  let code: { type: TokenType; token: string }[][] = [];

  const test = async () => {
    let res: any = await invoke("test", {
      file: "c:/Users/benja/Documents/Coding/Apps/CodeForge/src-tauri/test/main.rs",
    });
    code = res.tokens;
    console.log(code);
  };

  test();
</script>

<button on:click={test}>Test</button>
<div class="editor">
  {#each code as block}
    <div class="block">
      <span class="block-span">
        {#each block as token}
          <span class="token" style="color: {test_style.tokenColors[token.type]};">{token.token}</span>
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
    overflow-y: auto;
    height: 100%;
  }

  .block {
    display: flex;
  }

  .block-span {
    display: flex;
    flex-wrap: nowrap;
    height: calc(1rem + 2px);
  }

  .token {
    white-space: pre;
  }
</style>
