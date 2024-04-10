<script type="ts">
  import { Window } from "@tauri-apps/api/window";

  const appWindow = new Window("main");
</script>

<div class="titlebar">
  <div data-tauri-drag-region class="space"></div>
  <button class="titlebar-button" id="titlebar-minimize" on:click={appWindow.minimize}>
    <img src="icons/action/minimize.svg" alt="minimize" style="scale: 1.1;" />
  </button>
  <button class="titlebar-button" id="titlebar-maximize" on:click={appWindow.toggleMaximize}>
    <img src="icons/action/maximize.svg" alt="maximize" />
  </button>
  <button class="titlebar-button" id="titlebar-close" on:click={appWindow.close}>
    <img src="icons/action/close.svg" alt="close" style="scale: 1.1;" />
  </button>
</div>

<style>
  .titlebar {
    height: 35px;
    user-select: none;
    display: flex;
    justify-content: flex-end;
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
  }
  .space {
    flex: 1;
  }
  .titlebar-button {
    display: inline-flex;
    justify-content: center;
    align-items: center;
    width: 35px;
    height: 35px;
    user-select: none;
    -webkit-user-select: none;
    border: none;
    background: transparent;
    padding: 0;
  }
  .titlebar-button:hover {
    background: var(--color-secondary);
  }
  .titlebar-button img {
    width: 50%;
    height: 50%;
    object-fit: contain;
  }
</style>
