<script lang="ts">
  import type { ComponentAndProps as Component } from "../../utils";
  export let tabs: { name: string; component: Component | string }[] = [];

  let activeTab = tabs[0];
</script>

<div class="container">
  <div class="tabs-container">
    {#each tabs as tab}
      <button
        class="tab"
        class:active={activeTab === tab}
        on:click={() => (activeTab = tab)}
      >
        <p>{tab.name}</p>
      </button>
    {/each}
  </div>
  {#if typeof activeTab.component === "string"}
    {@html `<${activeTab.component} />`}
  {:else}
    <svelte:component this={activeTab.component.component} {...activeTab.component.props} />
  {/if}
</div>

<style>
  .container {
    display: grid;
    grid-template-rows: 2em 1fr;
    background-color: var(--color-background);
  }

  .tabs-container {
    display: flex;
    flex-direction: row;
  }

  .tab {
    color: var(--color-primary);
    cursor: pointer;
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 0 1em;
    border-radius: 0.4em;
    border: none;
    background-color: var(--color-background);
  }

  .tab p {
    text-wrap: nowrap;
  }

  .tab.active {
    background-color: var(--color-tertiary);
  }
</style>
