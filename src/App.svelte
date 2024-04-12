<script lang="ts">
  import SplitView from "./lib/components/layout/SplitView.svelte";
  import Titlebar from "./lib/components/main/Titlebar.svelte";
</script>

<div class="app">
  <header>
    <Titlebar />
  </header>
  <main>
    <SplitView size="l150">
      <div slot="first" class="sidebar"></div>
      <div slot="second" class="content"></div>
    </SplitView>
  </main>
</div>

<style>
  :global(:root) {
    --color-accent: #e6b450;
    --color-primary: #c5c5c5;
    --color-secondary: #383d45;
    --color-background: #0b0e14;

    --header-height: 35px;
  }

  .app {
    height: 100%;
    background-color: var(--color-background);
  }

  main {
    height: calc(100% - var(--header-height));
  }
</style>
