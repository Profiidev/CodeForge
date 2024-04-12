<script type="ts">
  import { Window } from "@tauri-apps/api/window";
  import Icon from "../basic/Icon.svelte";

  const appWindow = new Window("main");
</script>

<div class="titlebar">
  <div data-tauri-drag-region class="space"></div>
  <button
    class="titlebar-button normal"
    id="titlebar-minimize"
    on:click={appWindow.minimize}
  >
    <Icon src={"icons/action/minimize.svg"} color={"var(--color-primary)"} />
  </button>
  <button
    class="titlebar-button normal"
    id="titlebar-maximize"
    on:click={appWindow.toggleMaximize}
  >
    <Icon src={"icons/action/maximize.svg"} color={"var(--color-primary)"} />
  </button>
  <button
    class="titlebar-button exit"
    id="titlebar-close"
    on:click={appWindow.close}
  >
    <Icon src={"icons/action/close.svg"} color={"var(--color-primary)"} />
  </button>
</div>

<style>
  .titlebar {
    height: var(--header-height);
    user-select: none;
    display: flex;
    justify-content: flex-end;
  }
  .space {
    flex: 1;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: var(--header-height);
    height: var(--header-height);
    user-select: none;
    -webkit-user-select: none;
    border: none;
    background: transparent;
    padding: 0;
  }
  .normal:hover {
    background: var(--color-secondary);
  }
  .exit:hover {
    background: red;
  }
</style>
