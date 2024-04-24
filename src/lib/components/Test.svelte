<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  let time = 0;
  const test = async () => {
    let start = performance.now();
    await invoke("greet", {
      file: "file:///c:/Users/benja/Documents/Coding/Apps/CodeForge/src-tauri/src/main.rs",
    }).then((res) => console.log(res));
    time = performance.now() - start;
  };
</script>

<div class="test" style="background-color: var(--color-secondary);"></div>
<button on:click={test}>Test</button>
<p style="color: white;">{time}</p>