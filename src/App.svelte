<script lang="ts">
  import Titlebar from "./lib/components/main/Titlebar.svelte";
</script>

<div class="app">
  <header>
    <Titlebar />
  </header>
  <main class="container">
  </main>
</div>

<style>
  :global(:root) {
    --color-accent: #E6B450;
    --color-primary: #C5C5C5;
    --color-secondary: #383D45;
    --color-background: #0B0E14;
  }

  .app {
    height: 100%;
    background-color: var(--color-background);
  }
</style>
