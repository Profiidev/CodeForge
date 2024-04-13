<script lang="ts">
  import Test from "./lib/components/Test.svelte";
  import SplitView from "./lib/components/layout/SplitView.svelte";
  import TabsView from "./lib/components/layout/TabsView.svelte";
  import Titlebar from "./lib/components/main/Titlebar.svelte";
  import { component } from "./lib/utils";
</script>

<div class="app">
  <header>
    <Titlebar />
  </header>
  <main>
    <SplitView
      size={[150]}
      components={[component(Test, {}), component(TabsView, {})]}
    />
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
