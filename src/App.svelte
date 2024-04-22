<script lang="ts">
  import Test from "./lib/components/Test.svelte";
  import SplitView from "./lib/components/layout/SplitView.svelte";
  import TabsView from "./lib/components/layout/TabsView.svelte";
  import Editor from "./lib/components/main/Editor.svelte";
  import Titlebar from "./lib/components/main/Titlebar.svelte";
  import { component as c } from "./lib/utils";
</script>

<div class="app">
  <header>
    <Titlebar />
  </header>
  <main>
    <SplitView
      size={[150]}
      components={[
        c(Test, {}),
        c(SplitView, {
          size: [150],
          components: [
            c(TabsView, {
              tabs: [
                { name: "Tab 1", component: c(Editor, {}) },
                { name: "Tab 2", component: "button" },
              ],
            }),
            c(Test, {}),
          ],
          direction: "b",
        }),
      ]}
    />
  </main>
</div>

<style>
  :global(:root) {
    --color-accent: #e6b450;
    --color-primary: #c5c5c5;
    --color-secondary: #383d45;
    --color-tertiary: #2b3036;
    --color-background: #0b0e14;

    --header-height: 35px;

    --font-family: "JetBrains Mono", monospace;
  }

  .app {
    height: 100%;
    background-color: var(--color-background);
    font-family: var(--font-family);
    user-select: none;
  }

  main {
    height: calc(100% - var(--header-height));
  }
</style>
