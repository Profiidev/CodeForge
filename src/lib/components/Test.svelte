<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  const test = () => {
    invoke("greet", {
      file: "file:///c:/Users/benja/Documents/Coding/Apps/CodeForge/src-tauri/src/main.rs",
    }).then((res) => console.log(res));
  };
</script>

<div class="test" style="background-color: var(--color-secondary);"></div>
<button on:click={test}>Test</button>
